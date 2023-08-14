pub mod config;
pub mod handler;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard, Weak},
    thread,
};

use lazy_static::lazy_static;

use log::info;
use ms_hostcall::types::{IsolationID as IsolID, ServiceName};

use crate::{
    logger,
    metric::MetricBucket,
    service::{Service, ServiceLoader},
    utils::gen_new_id,
};
use config::IsolationConfig;

type IsolTable = HashMap<IsolID, Weak<Isolation>>;
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

impl Drop for IsolationInner {
    fn drop(&mut self) {
        while !self.modules.is_empty() {
            self.modules.clear()
        }
    }
}

pub struct Isolation {
    id: IsolID,
    loader: Arc<ServiceLoader>,
    pub metric: Arc<MetricBucket>,
    app_names: Vec<ServiceName>,

    inner: Mutex<IsolationInner>,
}

impl Isolation {
    pub fn new(config: IsolationConfig) -> Arc<Self> {
        let new_id = gen_new_id();
        logger::info!("start build isolation_{new_id}");

        let metric = Arc::from(MetricBucket::new());

        let loader = Arc::from(ServiceLoader::new(new_id, Arc::clone(&metric)).register(&config));

        let isol = Arc::from(Self {
            id: new_id,
            loader,
            metric,
            app_names: config.apps.iter().map(|app| app.0.clone()).collect(),

            inner: Mutex::new(IsolationInner::default()),
        });

        get_isol_table().insert(isol.id, Arc::downgrade(&isol));

        isol
    }

    pub fn preload(&self, mut config: IsolationConfig) {
        let mut modules = config.services;
        modules.append(&mut config.apps);

        for service in modules {
            let svc_name = service.0;
            self.service_or_load(&svc_name);
        }
    }

    pub fn inner_access(&self) -> MutexGuard<'_, IsolationInner> {
        self.inner.lock().unwrap()
    }

    pub fn service_or_load(&self, name: &ServiceName) -> Arc<Service> {
        let mut isol_inner = self.inner_access();
        match isol_inner.modules.get(name) {
            Some(svc) => Arc::clone(svc),
            None => {
                info!("first load {}.", name);
                let svc = self.loader.load_service(name);
                isol_inner.modules.insert(name.to_owned(), Arc::clone(&svc));
                svc
            }
        }
    }

    pub fn run(&self) -> Result<(), ()> {
        let isol = {
            let binding = get_isol_table();
            let isol = binding.get(&self.id).expect("isol doesn't exist?");
            isol.upgrade().expect("isol doesn't exist?")
        };
        let app_names = self.app_names.clone();

        let thread_builder =
            thread::Builder::new().name(format!("isol-{}-{}", self.id, app_names.get(0).unwrap()));

        let handler = thread_builder
            .spawn(move || {
                for app in app_names {
                    let app = isol.service_or_load(&app);
                    let result = app.run();
                    result?
                }
                Ok(())
            })
            .expect("thread spawn failed");

        handler.join().expect("Join isolation app-thread failed")
    }
}

impl Drop for Isolation {
    fn drop(&mut self) {
        get_isol_table().remove(&self.id).unwrap();
    }
}
