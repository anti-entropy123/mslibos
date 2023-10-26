use core::panic;

use ms_hostcall::{
    types::{IsolationID, MetricEvent, MetricFunc, NetdevName},
    CommonHostCall, HostCallID,
};

use crate::{isolation::get_isol, logger};

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
    let isol = get_isol(isol_id).expect("isol don't exist?");

    let addr = match hc_id {
        HostCallID::Common(CommonHostCall::Metric) => {
            let f: MetricFunc = metric_handler;
            f as usize
        }
        _ => {
            let svc_name = hc_id.belong_to();
            logger::debug!(
                "hostcall_{} belong to service: {}",
                hc_id.to_string(),
                svc_name
            );

            let service = isol.service_or_load(&svc_name).unwrap_or_else(|e| {
                panic!("need find: {}, need load: {}, err: {}", hc_id, svc_name, e)
            });

            let symbol = service
                .interface::<fn()>(&hc_id.to_string())
                .unwrap_or_else(|| {
                    panic!(
                        "not found interface \"{}\" in service \"{}\"",
                        hc_id,
                        hc_id.belong_to()
                    )
                });
            *symbol as usize
        }
    };

    log::debug!("interface '{}' addr = 0x{:x}", hc_id, addr);
    addr
}

fn metric_handler(isol_id: IsolationID, event: MetricEvent) -> Result<(), ()> {
    let isol = get_isol(isol_id).expect("isol don't exist?");
    isol.metric.mark(event);

    Ok(())
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

        let isol = Isolation::new(&IsolationConfig {
            services,
            apps: vec![(
                "hello1".to_owned(),
                utils::TARGET_DEBUG_PATH.join("libhello_world.so"),
            )],
            groups: Default::default(),
        });
        // isol_table.insert(1, Arc::clone(&isol));
        isol
    };

    let hostcall_id = HostCallID::Common(ms_hostcall::CommonHostCall::Write);
    let addr = unsafe { find_host_call(1, hostcall_id) };

    let fs_svc = isol
        .service_or_load(&"fdtab".to_string())
        .expect("service not found?");

    let symbol = fs_svc
        .interface::<fn()>("write")
        .expect("not found interface 'write'");

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
