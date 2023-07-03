mod elf_service;
mod rust_service;

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use libloading::Symbol;

use elf_service::ELFService;
use ms_hostcall::types::{IsolationID, ServiceName};

use crate::{
    isolation::config::IsolationConfig,
    logger,
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
        for app in &config.apps {
            self.registered.insert(app.0.clone(), app.1.clone());
        }

        for svc in &config.services {
            self.registered.insert(svc.0.clone(), svc.1.clone());
        }
        self
    }

    fn load(&self, name: &ServiceName) -> Arc<Service> {
        let lib_path = self
            .registered
            .get(name)
            .unwrap_or_else(|| panic!("unregistered library: {}", name));

        let service = Service::new(name, lib_path, self.metric.new_svc_metric(name.clone()));
        service.init(self.isol_id);
        Arc::from(service)
    }

    pub fn load_service(&self, name: &ServiceName) -> Arc<Service> {
        self.metric.mark(MetricEvent::LoadService);
        self.load(name)
    }
}

pub enum Service {
    ElfService(elf_service::ELFService),
    RustService(rust_service::RustService),
}

impl Service {
    fn new(name: &str, filename: &PathBuf, metric: Arc<SvcMetricBucket>) -> Self {
        logger::debug!("Service::new, name={name}");
        Self::ElfService(ELFService::new(name, filename, metric))
    }
    fn init(&self, isol_id: IsolationID) {
        match self {
            Service::ElfService(svc) => svc.init(isol_id),
            Service::RustService(svc) => svc.init(isol_id),
        }
    }
    pub fn run(&self) -> Result<(), ()> {
        match self {
            Service::ElfService(svc) => svc.run(),
            Service::RustService(svc) => svc.run(),
        }
    }
    pub fn interface<T>(&self, name: &str) -> Option<Symbol<T>> {
        match self {
            Service::ElfService(svc) => svc.symbol(name),
            Service::RustService(svc) => Some(svc.symbol(name)),
        }
    }
    pub fn name(&self) -> ServiceName {
        match self {
            Service::ElfService(svc) => svc.name.to_owned(),
            Service::RustService(svc) => svc.name.to_owned(),
        }
    }
}

// impl Drop for Service {
//     fn drop(&mut self) {
//         match self {
//             Service::ElfService(svc) => svc.,
//             Service::RustService(_) => todo!(),
//         }
//     }
// }
