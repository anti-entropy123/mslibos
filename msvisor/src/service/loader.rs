use std::{
    collections::HashMap,
    fmt::Display,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use ms_hostcall::types::{IsolationID, ServiceName};
use nix::libc::Lmid_t;

use crate::{
    isolation::config::IsolationConfig,
    metric::{MetricBucket, MetricEvent},
};

use super::Service;

#[derive(Default)]
pub struct Namespace(Lmid_t);

impl Namespace {
    pub fn as_lmid_t(&self) -> Lmid_t {
        self.0
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct ServiceLoader {
    isol_id: IsolationID,
    registered: HashMap<ServiceName, PathBuf>,
    metric: Arc<MetricBucket>,
    namespace: OnceLock<Namespace>,
}

impl ServiceLoader {
    pub fn new(isol_id: IsolationID, metric: Arc<MetricBucket>) -> Self {
        Self {
            isol_id,
            registered: HashMap::new(),
            namespace: OnceLock::new(),
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
        #[cfg(feature = "namespace")]
        self.registered.insert(
            "libc".to_owned(),
            PathBuf::from("/lib/x86_64-linux-gnu/libc.so.6"),
        );
        self
    }

    fn load(&self, name: &ServiceName) -> Arc<Service> {
        let lib_path = self
            .registered
            .get(name)
            .unwrap_or_else(|| panic!("unregistered library: {}", name));

        let service = Service::new(
            name,
            lib_path,
            self.namespace.get(),
            self.metric.new_svc_metric(name.clone()),
        );
        self.namespace.get_or_init(|| service.namespace());

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
