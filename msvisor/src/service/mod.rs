mod elf_service;
mod rust_service;

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use libloading::Symbol;

use elf_service::ELFService;
use ms_hostcall::types::{IsolationID, ServiceName};

use crate::{
    isolation::IsolationConfig,
    metric::{MetricBucket, MetricEvent, SvcMetricBucket},
};

pub struct ServiceLoader {
    isol_id: IsolationID,
    registered: HashMap<ServiceName, PathBuf>,
    metric: Arc<MetricBucket>,
}

impl ServiceLoader {
    pub fn new(isol_id: IsolationID, metric: Arc<MetricBucket>) -> Self {
        Self {
            isol_id,
            registered: HashMap::new(),
            metric,
        }
    }

    pub fn register(mut self, config: &IsolationConfig) -> Self {
        self.registered
            .insert(config.app.0.clone(), config.app.1.clone());

        for svc in &config.services {
            self.registered.insert(svc.0.clone(), svc.1.clone());
        }
        self
    }

    fn load(&self, name: &ServiceName) -> Arc<Service> {
        let lib_path = self.registered.get(name).expect("unregistered library!");

        let service = Service::new(name, lib_path, self.metric.new_svc_metric(name.clone()));
        service.init(self.isol_id);
        Arc::from(service)
    }

    pub fn load_app(&self, name: &ServiceName) -> Arc<Service> {
        self.load(name)
    }

    pub fn load_service(&self, name: &ServiceName) -> Arc<Service> {
        self.metric.mark(MetricEvent::LoadService);
        self.load(name)
    }
}

// impl Default for ServiceLoader {
//     fn default() -> Self {
//         Self::new()
//     }
// }

pub enum Service {
    ElfService(elf_service::ELFService),
    RustService(rust_service::RustService),
}

impl Service {
    fn new(name: &str, filename: &PathBuf, metric: Arc<SvcMetricBucket>) -> Self {
        Self::ElfService(ELFService::new(name, filename, metric))
    }
    fn init(&self, isol_id: IsolationID) {
        match self {
            Service::ElfService(svc) => svc.init(isol_id),
            Service::RustService(svc) => svc.init(isol_id),
        }
    }
    pub fn run(&self) {
        match self {
            Service::ElfService(svc) => svc.run(),
            Service::RustService(svc) => svc.run(),
        }
    }
    pub fn interface<T>(&self, name: &str) -> Symbol<T> {
        match self {
            Service::ElfService(svc) => svc.symbol(name),
            Service::RustService(svc) => svc.symbol(name),
        }
    }
    pub fn name(&self) -> ServiceName {
        match self {
            Service::ElfService(svc) => svc.name.to_owned(),
            Service::RustService(svc) => svc.name.to_owned(),
        }
    }
}
