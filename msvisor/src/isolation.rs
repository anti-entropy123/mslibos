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
    types::{HostWriteFunc, IsolationID as IsolID, ServiceName},
    CommonHostCall, HostCallID, IsolationContext,
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
        let mut loader = ServiceLoader::new().register(config.app.0.clone(), config.app.1);
        for svc in config.services {
            loader = loader.register(svc.0, svc.1);
        }

        let new_id = gen_new_id();

        let user_app = loader.load_service(
            IsolationContext {
                isol_id: new_id,
                find_handler: find_host_call as usize,
            },
            &config.app.0,
        );

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
    logger::debug!("find_host_call, isol_id={:x}, call_id={:?}", isol_id, hc_id);
    let isol = {
        let isol_table = ISOL_TABLE.lock().unwrap();
        Arc::clone(isol_table.get(&isol_id).unwrap())
    };

    let addr = match hc_id {
        HostCallID::Common(CommonHostCall::Write) => {
            let mut isol_inner = isol.inner_access();
            let fs_module = match isol_inner.modules.get("fs") {
                Some(fs) => Arc::clone(fs),
                None => {
                    let fs = isol.loader.load_service(
                        IsolationContext {
                            isol_id,
                            find_handler: find_host_call as usize,
                        },
                        &"fs".to_owned(),
                    );
                    isol_inner.modules.insert("fs".to_string(), Arc::clone(&fs));
                    fs
                }
            };
            let func: HostWriteFunc = *fs_module.interface("host_write");
            func as usize
        }
        _ => todo!(),
    };
    log::debug!("host_write addr = 0x{:x}", addr);
    addr
}
