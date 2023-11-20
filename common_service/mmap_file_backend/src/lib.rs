use std::{mem::MaybeUninit, os::raw::c_void, sync::Mutex};

use lazy_static::lazy_static;
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

struct RegisterdMemRegion {
    uffd: Uffd,
    start_addr: usize,
    src_fd: Fd,
}

#[no_mangle]
pub fn file_page_fault_handler() -> LibOSResult<()> {
    let mut page = Box::new(MaybeUninit::<Page>::uninit());

    loop {
        let regions = REGISTERD_REGIONS.lock().unwrap();
        let uffds: Vec<_> = regions.iter().map(|region| &region.uffd).collect();

        let mut pollfds: Vec<_> = uffds
            .iter()
            .map(|uffd| PollFd::new(uffd, PollFlags::POLLIN))
            .collect();

        let nready = poll(pollfds.as_mut_slice(), -1).expect("poll");
        // let revents = pollfd.revents().unwrap();

        let uffd = uffds.get(0).unwrap();
        let event = uffd
            .read_event()
            .expect("read uffd_msg")
            .expect("uffd_msg ready");

        if let Event::Pagefault { addr, .. } = event {
            // Display info about the page-fault event

            println!("UFFD_EVENT_PAGEFAULT event: {:?}", event);

            // Copy the page pointed to by 'page' into the faulting region. Vary the contents that are
            // copied in, so that it is more obvious that each fault is handled separately.

            for c in unsafe {
                std::slice::from_raw_parts_mut::<u8>(page.as_mut_ptr() as usize as *mut u8, 0x1000)
            } {
                *c = b'A';
            }

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

            println!("(uffdio_copy.copy returned {})", copy);
        } else {
            panic!("Unexpected event on userfaultfd");
        }
    }

    Ok(())
}

#[no_mangle]
pub fn register_file_backend(mm_region: &mut [c_void], file_fd: Fd) -> LibOSResult<()> {
    let uffd = UffdBuilder::new()
        .close_on_exec(true)
        .non_blocking(true)
        .user_mode_only(true)
        .create()
        // If have error: `OpenDevUserfaultfd(Os { code: 13, kind: PermissionDenied, message: "Permission denied" })`
        // ,use this command:
        //    `setfacl -m u:${USER}:rw /dev/userfaultfd`
        .expect("uffd creation failed");

    uffd.register(mm_region.as_mut_ptr(), mm_region.len())
        .expect("register failed");

    REGISTERD_REGIONS.lock().unwrap().push(RegisterdMemRegion {
        uffd,
        start_addr: mm_region.as_ptr() as usize,
        src_fd: file_fd,
    });

    Ok(())
}
