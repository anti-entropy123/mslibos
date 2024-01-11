mod elf_service;
mod loader;
#[cfg(feature = "serviceV2")]
mod rust_service;

use std::{collections::BTreeMap, sync::Arc};

use libloading::{Library, Symbol};

use elf_service::WithLibOSService;
pub use loader::ServiceLoader;
use ms_hostcall::types::{IsolationID, ServiceName};

use crate::{logger, metric::SvcMetricBucket};

use self::loader::Namespace;

pub enum Service {
    WithLibOSService(elf_service::WithLibOSService),
    #[cfg(feature = "serviceV2")]
    RustService(rust_service::RustService),
}

impl Service {
    fn new(name: &str, lib: Arc<Library>, metric: Arc<SvcMetricBucket>) -> Self {
        logger::debug!("Service::new, name={name}");
        Self::WithLibOSService(WithLibOSService::new(name, lib, metric))
    }
    fn init(&self, isol_id: IsolationID) -> anyhow::Result<()> {
        match self {
            Service::WithLibOSService(svc) => svc.init(isol_id),
            #[cfg(feature = "serviceV2")]
            Service::RustService(svc) => svc.init(isol_id),
        }
    }
    pub fn run(&self, args: &BTreeMap<String, String>) -> Result<(), String> {
        match self {
            Service::WithLibOSService(svc) => svc.run(args),
            #[cfg(feature = "serviceV2")]
            Service::RustService(svc) => svc.run(),
        }
    }
    pub fn interface<T>(&self, name: &str) -> Option<Symbol<T>> {
        match self {
            Service::WithLibOSService(svc) => svc.symbol(name),
            #[cfg(feature = "serviceV2")]
            Service::RustService(svc) => Some(svc.symbol(name)),
        }
    }
    pub fn name(&self) -> ServiceName {
        match self {
            Service::WithLibOSService(svc) => svc.name(),
            #[cfg(feature = "serviceV2")]
            Service::RustService(svc) => svc.name.to_owned(),
        }
    }
    pub fn namespace(&self) -> Namespace {
        match self {
            Service::WithLibOSService(svc) => svc.namespace(),
            #[cfg(feature = "serviceV2")]
            Service::RustService(_) => todo!(),
        }
    }
}

// impl Drop for Service {
//     fn drop(&mut self) {
//         match self {
//             Service::WithLibOSService(svc) => svc.,
//             Service::RustService(_) => todo!(),
//         }
//     }
// }
