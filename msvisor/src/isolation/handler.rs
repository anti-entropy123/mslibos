use ms_hostcall::{
    types::{IsolationID, NetdevName},
    HostCallID,
};

use crate::{isolation::ISOL_TABLE, logger};

/// # Safety
/// This is unsafe because it it be a callback function used to lookup the address of
/// hostcall function symbols, and it should be only invocated by service modules.
///
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn find_host_call(isol_id: IsolationID, hc_id: HostCallID) -> usize {
    // let id = HostCallID::Common(CommonHostCall::Write);
    // thread::sleep(Duration::from_secs(1));

    logger::debug!(
        "find_host_call, isol_id={:x}, call_id={:?}, call_name={}",
        isol_id,
        hc_id,
        hc_id.to_string()
    );
    let isol = {
        let isol_table = ISOL_TABLE.lock().unwrap();
        isol_table
            .get(&isol_id)
            .unwrap()
            .upgrade()
            .expect("isolation already stopped?")
    };

    let svc_name = hc_id.belong_to();
    logger::debug!(
        "hostcall_{} belong to service: {}",
        hc_id.to_string(),
        svc_name
    );

    let service = isol.service_or_load(&svc_name);
    let symbol = service
        .interface::<fn()>(&hc_id.to_string())
        .unwrap_or_else(|| {
            panic!(
                "not found interface {} in service {}",
                hc_id,
                hc_id.belong_to()
            )
        });
    let addr = *symbol as usize;

    log::debug!("host_write addr = 0x{:x}", addr);
    addr
}

#[test]
fn find_host_call_test() {
    // logger::init();
    use crate::{
        isolation::{Isolation, IsolationConfig},
        utils,
    };

    let isol = {
        // let mut isol_table = ISOL_TABLE.lock().unwrap();
        let services = {
            let mut s = Vec::new();
            s.push((
                "fdtab".to_owned(),
                utils::TARGET_DEBUG_PATH.join("libfdtab.so"),
            ));
            s
        };
        log::debug!("services={:#?}", services);

        let isol = Isolation::new(IsolationConfig {
            services,
            app: (
                "hello1".to_owned(),
                utils::TARGET_DEBUG_PATH.join("libhello_world.so"),
            ),
        });
        // isol_table.insert(1, Arc::clone(&isol));
        isol
    };

    let hostcall_id = HostCallID::Common(ms_hostcall::CommonHostCall::Write);
    let addr = unsafe { find_host_call(1, hostcall_id) };

    let fs_svc = isol.service_or_load(&"fdtab".to_string());
    let symbol = fs_svc
        .interface::<fn()>("host_write")
        .expect("not found host_write");

    assert!(addr == *symbol as usize)
}

pub fn netdev_alloc_handler() -> Result<NetdevName, ()> {
    Err(())
}

/// A panic handler that should be registered into hostcalls.
///
/// ## TODO
/// This is a bad implementation because had better just make
/// threads of app exit gracefully. So maybe better call `pthread_exit`
/// rather than `panic!`.
///
/// ## Safety
/// It should only be invoked by panic_handler of ms_std.
pub unsafe extern "C" fn panic_handler() -> ! {
    panic!()
    // core::panic!()
}
