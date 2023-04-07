use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::{
    logger,
    metric::MetricBucket,
    service::{Service, ServiceLoader},
    utils::gen_new_id,
};

use ms_hostcall::{
    types::{IsolationID as IsolID, ServiceName},
    HostCallID,
};

use lazy_static::lazy_static;

type IsolTable = HashMap<IsolID, Arc<Isolation>>;
lazy_static! {
    pub static ref ISOL_TABLE: Mutex<IsolTable> = Mutex::new(HashMap::new());
}

fn get_isol_table() -> MutexGuard<'static, IsolTable> {
    ISOL_TABLE.lock().unwrap()
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
    pub metric: Arc<MetricBucket>,

    inner: Mutex<IsolationInner>,
}

impl Isolation {
    pub fn new(config: IsolationConfig) -> Arc<Self> {
        let new_id = gen_new_id();
        logger::info!("start build isolation_{new_id}");

        let metric = Arc::from(MetricBucket::new());

        let loader = ServiceLoader::new(new_id, Arc::clone(&metric)).register(&config);

        let user_app = loader.load_app(&config.app.0);

        let isol = Arc::from(Self {
            id: new_id,
            user_app,
            loader,
            metric,
            inner: Mutex::new(IsolationInner::default()),
        });

        get_isol_table().insert(isol.id, Arc::clone(&isol));

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
                let fs = self.loader.load_service(name);
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
    // use libloading::Symbol;

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

    let hostcall_id = HostCallID::Common(ms_hostcall::CommonHostCall::Write);
    let addr = unsafe { find_host_call(1, hostcall_id) };
    let binding = isol.service_or_load(&"fs".to_string());
    let symbol = binding.interface::<fn()>("host_write");

    assert!(addr == *symbol as usize)
}

/// A panic handler that should be registered into hostcalls.
///
/// # Safety
/// It should only be invoked by panic_handler of ms_std.
pub unsafe extern "C" fn panic_handler() -> ! {
    panic!()
}
