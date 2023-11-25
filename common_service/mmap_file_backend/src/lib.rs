use std::{
    ffi::c_void,
    mem::{ManuallyDrop, MaybeUninit},
    os::fd::{AsFd, BorrowedFd, RawFd},
    slice::from_raw_parts_mut,
    sync::{Mutex, MutexGuard, RwLock},
    u64,
};

use lazy_static::lazy_static;
use nix::{
    fcntl::{fcntl, FcntlArg, OFlag},
    sys::epoll::{Epoll, EpollCreateFlags, EpollEvent, EpollFlags},
};
use userfaultfd::{Event, Uffd, UffdBuilder};

use ms_hostcall::{err::LibOSResult, types::Fd};
use ms_std::{fs::File, io::Read, libos::libos, println};

#[repr(C, align(4096))]
struct Page([u8; PAGE_SIZE]);

const PAGE_SIZE: usize = 0x1000;

struct NotifyPipe {
    recevier: RawFd,
    sender: RawFd,
}

impl NotifyPipe {
    fn new() -> Self {
        let (recevier, sender) = nix::unistd::pipe().expect("make os pipe failed");
        let flags = fcntl(sender, FcntlArg::F_GETFL).expect("get flags failed");
        fcntl(
            sender,
            FcntlArg::F_SETFL(OFlag::from_bits(flags).unwrap() | OFlag::O_NONBLOCK),
        )
        .expect("set non block failed");

        Self { recevier, sender }
    }

    fn consume(&self) {
        let mut buf = [0u8];
        unsafe { nix::libc::read(self.recevier, buf.as_mut_ptr() as usize as *mut c_void, 1) };
    }

    fn notify(&self) {
        let buf = [0u8];
        unsafe { nix::libc::write(self.sender, buf.as_ptr() as usize as *mut c_void, 1) };
    }
}

impl Drop for NotifyPipe {
    fn drop(&mut self) {
        unsafe {
            nix::libc::close(self.recevier);
            nix::libc::close(self.sender);
        }
    }
}

lazy_static! {
    static ref REGISTER: Mutex<()> = Default::default();
    static ref REGISTERD_REGIONS: Mutex<Vec<RegisterdMemRegion>> = Default::default();
    static ref NOTIFY_PIPE: RwLock<Option<NotifyPipe>> = Default::default();
}

#[derive(Debug)]
struct RegisterdMemRegion {
    uffd: Uffd,
    start_addr: usize,
    src_fd: Fd,
}

fn read_at_offset(fd: Fd, offset: u32, page: *mut u8) {
    // Copy the page pointed to by 'page' into the faulting region. Vary the contents that are
    // copied in, so that it is more obvious that each fault is handled separately.
    let mut src_file = ManuallyDrop::new(File::from_raw_fd(fd));
    src_file.seek(offset);

    let page = unsafe { from_raw_parts_mut(page, PAGE_SIZE) };

    let read_size = src_file.read(page).expect("read file failed.");
    println!(
        "src_file aligned_offset={}, read {} bytes",
        offset, read_size
    );
}

fn do_page_fault(page: *mut u8, region: &RegisterdMemRegion) {
    let uffd = &region.uffd;
    let event = uffd
        .read_event()
        .expect("read uffd_msg")
        .expect("uffd_msg ready");

    if let Event::Pagefault { addr, .. } = event {
        println!(
            "UFFD_EVENT_PAGEFAULT event: {:?}, register_info: {:?}",
            event, region
        );
        let offset = addr as usize - region.start_addr;
        let aligned_offset = offset & (!PAGE_SIZE + 1);
        read_at_offset(region.src_fd, aligned_offset as u32, page);

        let dst = (addr as usize & !(PAGE_SIZE - 1)) as *mut c_void;
        let _copy = unsafe {
            uffd.copy(page as usize as *mut c_void, dst, PAGE_SIZE, true)
                .expect("uffd copy failed.")
        };

        // println!("(uffdio_copy.copy returned {})", copy);
    } else {
        panic!("Unexpected event on userfaultfd");
    }
}

fn init_notify_pipe() {
    let mut notify_pipe = NOTIFY_PIPE.write().unwrap();
    if notify_pipe.is_some() {
        panic!("notify_pipe has exist")
    }
    let pipe = NotifyPipe::new();
    *notify_pipe = Some(pipe);
}

#[no_mangle]
pub fn file_page_fault_handler() {
    let notify_fd =
        unsafe { BorrowedFd::borrow_raw(NOTIFY_PIPE.read().unwrap().as_ref().unwrap().recevier) };

    let mut page: Box<MaybeUninit<Page>> = Box::new(MaybeUninit::uninit());

    loop {
        let regions = REGISTERD_REGIONS.lock().unwrap();
        if regions.is_empty() {
            break;
        }

        let epoll = Epoll::new(EpollCreateFlags::empty()).expect("create epoll failed");

        let uffd_events: Vec<_> = regions
            .iter()
            .map(|region| region.uffd.as_fd())
            .enumerate()
            .collect();
        let notify_event = [(u64::MAX as usize, notify_fd.as_fd())];

        for (idx, fd) in uffd_events.iter().chain(notify_event.iter()) {
            epoll
                .add(fd, EpollEvent::new(EpollFlags::EPOLLIN, *idx as u64))
                .expect("add event failed");
        }

        let mut ready_events = [EpollEvent::empty()];
        epoll
            .wait(&mut ready_events, -1)
            .expect("epoll wait failed");
        // let revents = pollfd.revents().unwrap();

        if !ready_events[0].events().contains(EpollFlags::EPOLLIN) {
            continue;
        }
        let event_idx = ready_events[0].data();
        if let Some(region) = regions.get(event_idx as usize) {
            do_page_fault(page.as_mut_ptr() as usize as *mut u8, region);
        } else {
            drop(regions);
            let pipe = NOTIFY_PIPE.read().unwrap();
            let pipe = pipe.as_ref().expect("pipe not exist?");
            pipe.consume()
        }
    }

    let mut notify_pipe = NOTIFY_PIPE.write().unwrap();
    *notify_pipe = None;
    println!("page fault handler exit.");
}

fn acquire_regions_or_notify() -> MutexGuard<'static, std::vec::Vec<RegisterdMemRegion>> {
    match REGISTERD_REGIONS.try_lock() {
        Ok(regions) => regions,
        Err(_) => {
            let notify_pipe = NOTIFY_PIPE.read().unwrap();
            notify_pipe.as_ref().expect("notify has not init?").notify();
            REGISTERD_REGIONS.lock().unwrap()
        }
    }
}

#[no_mangle]
pub fn register_file_backend(mm_region: &mut [c_void], file_fd: Fd) -> LibOSResult<()> {
    let _lock = REGISTER.lock().unwrap();
    // If have error: `OpenDevUserfaultfd(Os { code: 13, kind: PermissionDenied, message: "Permission denied" })`
    // ,use this command:
    //    `setfacl -m u:${USER}:rw /dev/userfaultfd`
    let uffd = UffdBuilder::new()
        .close_on_exec(true)
        .non_blocking(true)
        .user_mode_only(true)
        .create()
        .expect("uffd creation failed");

    uffd.register(mm_region.as_mut_ptr(), mm_region.len())
        .expect("register failed");

    let mut regions = acquire_regions_or_notify();
    regions.push(RegisterdMemRegion {
        uffd,
        start_addr: mm_region.as_ptr() as usize,
        src_fd: file_fd,
    });

    if NOTIFY_PIPE.read().unwrap().is_none() {
        init_notify_pipe();

        libos!(spawn_fault_handler(
            ms_std::init_context::isolation_ctx().isol_id
        ))
        .expect("spawn_fault_handler failed.");
    }
    // println!("spawn_fault_handler successfully.");

    Ok(())
}

#[no_mangle]
pub fn unregister_file_backend(addr: usize) -> LibOSResult<()> {
    let _lock = REGISTER.lock().unwrap();
    let pipe = NOTIFY_PIPE.read().unwrap();
    let pipe = pipe.as_ref().expect("notify pipe not exist?");
    pipe.notify();
    let mut regions = acquire_regions_or_notify();

    for (idx, region) in (*regions).iter().enumerate() {
        if region.start_addr == addr {
            regions.remove(idx);
            break;
        }
    }

    Ok(())
}
