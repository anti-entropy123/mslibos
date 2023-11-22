use std::{
    mem::{ManuallyDrop, MaybeUninit},
    os::raw::c_void,
    slice::from_raw_parts_mut,
    sync::Mutex,
};

use lazy_static::lazy_static;
use ms_std::{fs::File, io::Read, libos::libos, println};
use nix::poll::{poll, PollFd, PollFlags};
use userfaultfd::{Event, Uffd, UffdBuilder};

use ms_hostcall::{err::LibOSResult, types::Fd};
pub use ms_std;

#[repr(C, align(4096))]
struct Page([u8; PAGE_SIZE]);

const PAGE_SIZE: usize = 0x1000;

lazy_static! {
    static ref REGISTERD_REGIONS: Mutex<Vec<RegisterdMemRegion>> = Default::default();
}

#[derive(Debug)]
struct RegisterdMemRegion {
    uffd: Uffd,
    start_addr: usize,
    src_fd: Fd,
}

#[no_mangle]
pub fn file_page_fault_handler() -> LibOSResult<()> {
    let mut page: Box<MaybeUninit<Page>> = Box::new(MaybeUninit::uninit());

    loop {
        let regions = REGISTERD_REGIONS.lock().unwrap();
        let uffds: Vec<_> = regions.iter().map(|region| &region.uffd).collect();

        let mut pollfds: Vec<_> = uffds
            .iter()
            .map(|uffd| PollFd::new(uffd, PollFlags::POLLIN))
            .collect();

        let _nready = poll(pollfds.as_mut_slice(), -1).expect("poll");
        // let revents = pollfd.revents().unwrap();

        let region = regions.get(0).unwrap();
        let uffd = &region.uffd;
        let event = uffd
            .read_event()
            .expect("read uffd_msg")
            .expect("uffd_msg ready");

        if let Event::Pagefault { addr, .. } = event {
            // println!(
            //     "UFFD_EVENT_PAGEFAULT event: {:?}, register_info: {:?}",
            //     event, region
            // );
            // Copy the page pointed to by 'page' into the faulting region. Vary the contents that are
            // copied in, so that it is more obvious that each fault is handled separately.
            let mut src_file = ManuallyDrop::new(File::from_raw_fd(region.src_fd));
            let offset = addr as usize - region.start_addr;
            let aligned_offset = offset & (!PAGE_SIZE + 1);

            src_file.seek(aligned_offset as u32);
            let page: &mut [u8] =
                unsafe { from_raw_parts_mut(page.as_mut_ptr() as usize as *mut u8, 0x1000) };

            let read_size = src_file.read(page).expect("read file failed.");
            // println!(
            //     "src_file aligned_offset={}, read {} bytes",
            //     aligned_offset, read_size
            // );

            let dst = (addr as usize & !(PAGE_SIZE - 1)) as *mut c_void;
            let copy = unsafe {
                uffd.copy(
                    page.as_mut_ptr() as usize as *mut c_void,
                    dst,
                    PAGE_SIZE,
                    true,
                )
                .expect("uffd copy")
            };

            // println!("(uffdio_copy.copy returned {})", copy);
        } else {
            panic!("Unexpected event on userfaultfd");
        }
    }
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
