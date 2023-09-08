mod elf_service;
mod loader;
mod rust_service;

use std::{path::PathBuf, sync::Arc};

use libloading::Symbol;

use elf_service::ELFService;
pub use loader::ServiceLoader;
use ms_hostcall::types::{IsolationID, ServiceName};

use crate::{logger, metric::SvcMetricBucket};

use self::loader::Namespace;

pub enum Service {
    ElfService(elf_service::ELFService),
    RustService(rust_service::RustService),
}

impl Service {
    fn new(
        name: &str,
        filename: &PathBuf,
        namespace: Option<&Namespace>,
        metric: Arc<SvcMetricBucket>,
    ) -> Self {
        logger::debug!("Service::new, name={name}");
        Self::ElfService(ELFService::new(name, filename, namespace, metric))
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
    pub fn namespace(&self) -> Namespace {
        match self {
            Service::ElfService(svc) => svc.namespace(),
            Service::RustService(_) => todo!(),
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
