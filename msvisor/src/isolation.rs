use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::{
    logger,
    service::{Service, ServiceLoader},
    utils::gen_new_id,
};

use ms_hostcall::{
    types::{IsolationID as IsolID, ServiceName},
    HostCallID,
};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ISOL_TABLE: Mutex<HashMap<IsolID, Arc<Isolation>>> = Mutex::new(HashMap::new());
}

#[derive(Default)]
pub struct IsolationInner {
    modules: HashMap<ServiceName, Arc<Service>>,
}

impl IsolationInner {}

pub struct IsolationConfig {
    pub services: Vec<(ServiceName, PathBuf)>,
    pub app: (ServiceName, PathBuf),
}

pub struct Isolation {
    id: IsolID,
    user_app: Arc<Service>,
    loader: ServiceLoader,

    inner: Mutex<IsolationInner>,
}

impl Isolation {
    pub fn new(config: IsolationConfig) -> Arc<Self> {
        logger::info!("start build isolation");
        let mut loader = ServiceLoader::new().register(config.app.0.clone(), config.app.1);
        for svc in config.services {
            loader = loader.register(svc.0, svc.1);
        }

        let new_id = gen_new_id();

        let user_app = loader.load_service(new_id, &config.app.0);

        let isol = Arc::from(Self {
            id: new_id,
            user_app,
            loader,
            inner: Mutex::new(IsolationInner::default()),
        });

        ISOL_TABLE
            .lock()
            .unwrap()
            .insert(isol.id, Arc::clone(&isol));

        isol
    }

    pub fn inner_access(&self) -> MutexGuard<'_, IsolationInner> {
        self.inner.lock().unwrap()
    }

    pub fn service_or_load(&self, name: &ServiceName) -> Arc<Service> {
        let mut isol_inner = self.inner_access();
        match isol_inner.modules.get(name) {
            Some(fs) => Arc::clone(fs),
            None => {
                let fs = self.loader.load_service(self.id, name);
                isol_inner.modules.insert(name.to_owned(), Arc::clone(&fs));
                fs
            }
        }
    }

    pub fn run(&self) {
        self.user_app.run();
        ISOL_TABLE.lock().unwrap().remove(&self.id);
    }
}

/// # Safety
/// This is unsafe because it it be a callback function used to lookup the address of
/// hostcall function symbols, and it should be only invocated by service modules.
///
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn find_host_call(isol_id: IsolID, hc_id: HostCallID) -> usize {
    // let id = HostCallID::Common(CommonHostCall::Write);
    logger::debug!(
        "find_host_call, isol_id={:x}, call_id={:?}, call_name={}",
        isol_id,
        hc_id,
        hc_id.to_string()
    );
    let isol = {
        let isol_table = ISOL_TABLE.lock().unwrap();
        Arc::clone(isol_table.get(&isol_id).unwrap())
    };
    let svc_name = hc_id.belong_to();
    logger::debug!(
        "hostcall_{} belong to service: {}",
        hc_id.to_string(),
        svc_name
    );

    let service = isol.service_or_load(&svc_name);
    let addr = *service.interface::<fn()>(&hc_id.to_string()) as usize;

    log::debug!("host_write addr = 0x{:x}", addr);
    addr
}

#[test]
fn find_host_call_test() {
    const TARGET_DIR: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../target/debug/libnative_fs.so"
    );
    let isol = {
        let mut isol_table = ISOL_TABLE.lock().unwrap();
        let isol = Isolation::new(IsolationConfig {
            services: Vec::from([("fs".to_owned(), PathBuf::from(TARGET_DIR))]),
            app: (String::new(), PathBuf::new()),
        });
        isol_table.insert(1, Arc::clone(&isol));
        isol
    };
    let hostcall_id = HostCallID::Common(CommonHostCall::Write);
    let addr = unsafe { find_host_call(1, hostcall_id) };

    assert!(
        addr == *isol
            .service_or_load(&"fs".to_string())
            .interface("host_write")
    )
}
