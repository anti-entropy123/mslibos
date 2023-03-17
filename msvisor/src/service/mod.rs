pub mod rust_service;

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use libloading::Symbol;

use ms_hostcall::types::{IsolationID, ServiceName};
use rust_service::RustService;

pub struct ServiceLoader {
    registered: HashMap<ServiceName, PathBuf>,
}

impl ServiceLoader {
    pub fn new() -> Self {
        Self {
            registered: HashMap::new(),
        }
    }

    pub fn register(mut self, service: ServiceName, lib_path: PathBuf) -> Self {
        self.registered.insert(service, lib_path);
        self
    }

    pub fn load_service(&self, isol_id: IsolationID, name: &ServiceName) -> Arc<Service> {
        let lib_path = self.registered.get(name).expect("unregistered library!");

        let service = Service::new(name, lib_path);
        service.init(isol_id);
        Arc::from(service)
    }
}

impl Default for ServiceLoader {
    fn default() -> Self {
        Self::new()
    }
}

pub enum Service {
    RustService(rust_service::RustService),
}

impl Service {
    fn new(name: &str, filename: &PathBuf) -> Self {
        Self::RustService(RustService::new(name, filename))
    }
    fn init(&self, isol_id: IsolationID) {
        match self {
            Service::RustService(svc) => svc.init(isol_id),
        }
    }
    pub fn run(&self) {
        match self {
            Service::RustService(svc) => svc.run(),
        }
    }
    pub fn interface<T>(&self, name: &str) -> Symbol<T> {
        match self {
            Service::RustService(svc) => svc.symbol(name),
        }
    }

    pub fn name(&self) -> ServiceName {
        match self {
            Service::RustService(svc) => svc.name.to_owned(),
        }
    }
}
