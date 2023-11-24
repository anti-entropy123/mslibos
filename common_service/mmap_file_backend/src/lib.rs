use std::{
    ffi::c_void,
    iter::zip,
    mem::{ManuallyDrop, MaybeUninit},
    os::fd::{BorrowedFd, RawFd},
    slice::from_raw_parts_mut,
    sync::{Mutex, RwLock},
};

use lazy_static::lazy_static;
use ms_std::{fs::File, io::Read, libos::libos, println};
use nix::{
    fcntl::{fcntl, FcntlArg, OFlag},
    libc::O_NONBLOCK,
    poll::{poll, PollFd, PollFlags},
};
use userfaultfd::{Event, Uffd, UffdBuilder};

use ms_hostcall::{err::LibOSResult, types::Fd};
pub use ms_std;

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
    static ref REGISTERD_REGIONS: Mutex<Vec<RegisterdMemRegion>> = Default::default();
    static ref NOTIFY_PIPE: RwLock<Option<NotifyPipe>> = Default::default();
}

#[derive(Debug)]
struct RegisterdMemRegion {
    uffd: Uffd,
    start_addr: usize,
    src_fd: Fd,
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
        // Copy the page pointed to by 'page' into the faulting region. Vary the contents that are
        // copied in, so that it is more obvious that each fault is handled separately.
        let mut src_file = ManuallyDrop::new(File::from_raw_fd(region.src_fd));
        let offset = addr as usize - region.start_addr;
        let aligned_offset = offset & (!PAGE_SIZE + 1);
        src_file.seek(aligned_offset as u32);

        let page: &mut [u8] = unsafe { from_raw_parts_mut(page, 0x1000) };

        let read_size = src_file.read(page).expect("read file failed.");
        println!(
            "src_file aligned_offset={}, read {} bytes",
            aligned_offset, read_size
        );

        let dst = (addr as usize & !(PAGE_SIZE - 1)) as *mut c_void;
        let _copy = unsafe {
            uffd.copy(
                page.as_mut_ptr() as usize as *mut c_void,
                dst,
                PAGE_SIZE,
                true,
            )
            .expect("uffd copy failed.")
        };

        // println!("(uffdio_copy.copy returned {})", copy);
    } else {
        panic!("Unexpected event on userfaultfd");
    }
}

fn init_notify_pipe() -> RawFd {
    let mut notify_pipe = NOTIFY_PIPE.write().unwrap();
    if notify_pipe.is_some() {
        panic!("notify_pipe has exist")
    }
    let pipe = NotifyPipe::new();
    let notify_fd = pipe.recevier;
    *notify_pipe = Some(pipe);
    notify_fd
}

#[no_mangle]
pub fn file_page_fault_handler() {
    let mut page: Box<MaybeUninit<Page>> = Box::new(MaybeUninit::uninit());

    let notify_fd = unsafe { BorrowedFd::borrow_raw(init_notify_pipe()) };
    let notify_pollfd = PollFd::new(&notify_fd, PollFlags::POLLIN);

    loop {
        let regions = REGISTERD_REGIONS.lock().unwrap();
        if regions.is_empty() {
            break;
        }

        let mut pollfds: Vec<PollFd> = {
            let mut pollfds: Vec<_> = regions
                .iter()
                .map(|region| PollFd::new(&region.uffd, PollFlags::POLLIN))
                .collect();
            pollfds.push(notify_pollfd);
            pollfds
        };
        let mut nready = poll(pollfds.as_mut_slice(), -1).expect("poll");
        // let revents = pollfd.revents().unwrap();

        for (pollfd, region) in zip(pollfds.iter(), regions.iter()) {
            if nready == 0 {
                break;
            }
            if !pollfd.revents().unwrap().contains(PollFlags::POLLIN) {
                continue;
            }
            nready -= 1;
            do_page_fault(page.as_mut_ptr() as usize as *mut u8, region);
        }

        if nready > 0 {
            drop(regions);
            println!("receive flush notify.");
            let pipe = NOTIFY_PIPE.read().unwrap();
            let pipe = pipe.as_ref().expect("pipe not exist?");
            pipe.consume()
        }
    }

    let mut notify_pipe = NOTIFY_PIPE.write().unwrap();
    *notify_pipe = None;
    println!("page fault handler exit.");
}

#[no_mangle]
pub fn register_file_backend(mm_region: &mut [c_void], file_fd: Fd) -> LibOSResult<()> {
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

    REGISTERD_REGIONS.lock().unwrap().push(RegisterdMemRegion {
        uffd,
        start_addr: mm_region.as_ptr() as usize,
        src_fd: file_fd,
    });

    libos!(spawn_fault_handler(
        ms_std::init_context::isolation_ctx().isol_id
    ))
    .expect("spawn_fault_handler failed.");
    // println!("spawn_fault_handler successfully.");

    Ok(())
}

#[no_mangle]
pub fn unregister_file_backend(addr: usize) -> LibOSResult<()> {
    let pipe = NOTIFY_PIPE.read().unwrap();
    let pipe = pipe.as_ref().expect("notify pipe not exist?");
    pipe.notify();
    let mut regions = REGISTERD_REGIONS.lock().unwrap();

    for (idx, region) in (*regions).iter().enumerate() {
        if region.start_addr == addr {
            regions.remove(idx);
            break;
        }
    }

    Ok(())
}
