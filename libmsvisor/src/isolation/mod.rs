pub mod config;
pub mod handler;

use std::{
    collections::{BTreeMap, HashMap},
    iter::zip,
    sync::{Arc, Mutex, MutexGuard, Weak},
    thread,
};

use anyhow::{anyhow, Ok};
use lazy_static::lazy_static;

use log::info;
use ms_hostcall::types::{
    IsolationID as IsolID,
    MetricEvent::{IsolBegin, IsolEnd, Mem},
    ServiceName,
};

#[cfg(feature = "enable_mpk")]
use std::{env, ffi::c_void, fs};

use crate::{
    logger,
    metric::MetricBucket,
    service::{Service, ServiceLoader},
    utils::gen_new_id,
};
#[cfg(feature = "enable_mpk")]
use crate::{mpk, utils};
use config::IsolationConfig;

use self::config::App;

type IsolTable = HashMap<IsolID, Weak<Isolation>>;
lazy_static! {
    pub static ref ISOL_TABLE: Mutex<IsolTable> = Mutex::new(HashMap::new());
}

fn get_isol_table() -> MutexGuard<'static, IsolTable> {
    ISOL_TABLE.lock().unwrap()
}

pub fn get_isol(handle: IsolID) -> anyhow::Result<Arc<Isolation>> {
    let isol_table = get_isol_table();
    Ok(isol_table
        .get(&handle)
        .ok_or_else(|| anyhow!("isol don't exsit. handle={}", handle))?
        .upgrade()
        .ok_or_else(|| {
            anyhow!(
                "upgrade failed. isolation already stopped? handle={}",
                handle
            )
        })?)
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
    pub id: IsolID,
    loader: ServiceLoader,
    pub metric: Arc<MetricBucket>,
    app_names: Vec<ServiceName>,
    groups: Vec<Vec<App>>,
    fs_image: Option<String>,
    #[cfg(feature = "enable_mpk")]
    _pkey: i32,

    inner: Mutex<IsolationInner>,
}

impl Isolation {
    pub fn new(config: &IsolationConfig) -> Arc<Self> {
        let new_id = gen_new_id();
        logger::info!("start build isolation_{new_id}");

        let metric = Arc::from(MetricBucket::new());
        metric.mark(IsolBegin);
        metric.mark(Mem);

        let loader = ServiceLoader::new(
            new_id,
            Arc::clone(&metric),
            config.with_libos.unwrap_or(true),
        )
        .register(config);

        let isol = Arc::from(Self {
            id: new_id,
            loader,
            metric,
            app_names: config.apps.iter().map(|app| app.0.clone()).collect(),
            groups: config
                .groups
                .iter()
                .map(|group| group.to_isolation())
                .collect(),
            fs_image: config.fs_image.clone(),
            #[cfg(feature = "enable_mpk")]
            _pkey: mpk::pkey_alloc(),

            inner: Mutex::new(IsolationInner::default()),
        });

        get_isol_table().insert(isol.id, Arc::downgrade(&isol));

        isol
    }

    pub fn preload(&self, config: &IsolationConfig) -> Result<(), anyhow::Error> {
        for service in config.all_modules() {
            let svc_name = &service.0;
            if self.app_names.contains(svc_name) {
                self.app_or_load(svc_name)?;
            } else {
                self.service_or_load(svc_name)?;
            }
            if self.app_names.contains(svc_name) {
                self.app_or_load(svc_name)?;
            } else {
                self.service_or_load(svc_name)?;
            }
        }

        Ok(())
    }

    pub fn inner_access(&self) -> MutexGuard<'_, IsolationInner> {
        self.inner.lock().unwrap()
    }

    pub fn service_or_load(&self, name: &ServiceName) -> Result<Arc<Service>, anyhow::Error> {
        let mut isol_inner = self.inner_access();
        match isol_inner.modules.get(name) {
            Some(svc) => Ok(Arc::clone(svc)),
            None => {
                info!("[service] first load {}.", name);
                let svc = self.loader.load_service(name)?;
                isol_inner.modules.insert(name.to_owned(), Arc::clone(&svc));
                Ok(svc)
            }
        }
    }

    pub fn app_or_load(&self, name: &ServiceName) -> Result<Arc<Service>, anyhow::Error> {
        let mut isol_inner = self.inner_access();
        let app = match isol_inner.modules.get(name) {
            Some(app) => Arc::clone(app),
            None => {
                info!("[app] first load {}.", name);
                let app = self.loader.load_app(name)?;
                isol_inner.modules.insert(name.to_owned(), Arc::clone(&app));
                app
            }
        };

        Ok(app)
    }

    fn run_as_sequence(&self) -> Result<(), anyhow::Error> {
        let args = {
            let mut args = BTreeMap::new();
            args.insert("id".to_owned(), 0.to_string());
            args
        };

        for app in &self.app_names {
            let app = self
                .app_or_load(app)
                .map_err(|e| anyhow!("load app failed: {e}"))?;

            let result = app.run(&args);
            result.map_err(|e| anyhow!("app_{} run failed, reason: {}", app.name(), e))?
        }

        Ok(())
    }

    fn run_group_in_parallel(&self, group: &[App]) -> Result<(), anyhow::Error> {
        let apps: Vec<_> = group
            .iter()
            .map(|app| self.app_or_load(&app.name))
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        thread::scope(|scope| {
            let mut join_handles = Vec::with_capacity(apps.len());

            for (app, app_config) in zip(apps, group) {
                let builder = thread::Builder::new().name(app.name());
                let app_result = builder.spawn_scoped(scope, move || {
                    app.run(&app_config.args)
                        .map_err(|e| anyhow!("app {} run failed. {}", app.name(), e))
                })?;
                join_handles.push(Some(app_result));
            }

            for handle in &mut join_handles {
                let handle = handle.take().unwrap();
                handle.join().expect("join failed?")?;
            }

            Ok(())
        })
    }

    pub fn run(&self) -> Result<(), anyhow::Error> {
        self.metric.mark(Mem);
        #[cfg(feature = "enable_mpk")] {
            let maps_str = std::fs::read_to_string("/proc/self/maps").unwrap();
            let segments = utils::parse_memory_segments(&maps_str).unwrap();
            let black_list = [
                std::env::current_exe()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned(),
                "/usr/lib/x86_64-linux-gnu/libc.so.6".to_owned(),
                "/usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2".to_owned(),
                "/usr/lib/x86_64-linux-gnu/libgcc_s.so.1".to_owned(),
                "[heap]".to_owned(),
                "[stack]".to_owned(),
                "[vdso]".to_owned(),
                "[vvar]".to_owned(),
            ];
            for segment in segments {
                if let Some(path) = segment.clone().path {
                    if black_list.iter().any(|need| path.contains(need)) {
                        mpk::pkey_mprotect(segment.start_addr as *mut std::ffi::c_void, segment.length, segment.perm, 0x1).unwrap();
                        logger::info!("{} (0x{:x}, 0x{:x}) set mpk success with right {:?}.", segment.path.unwrap(), segment.start_addr, segment.start_addr + segment.length, segment.perm);
                    }
                }
            }
        }
        #[cfg(feature = "namespace")]
        self.service_or_load(&"libc".to_owned())
            .map_err(|e| anyhow!("namespace feature, load libc failed: {e}"))?;

        if self.groups.is_empty() {
            self.run_as_sequence()
                .map_err(|e| anyhow!("run_as_sequence failed: {e}"))?
        } else {
            for group in &self.groups {
                self.run_group_in_parallel(group)
                    .map_err(|e| anyhow!("run_group_in_parallel failed: {e}"))?
            }
        };

        self.metric.mark(Mem);
        self.metric.mark(IsolEnd);
        Ok(())
    }
}

impl Drop for Isolation {
    fn drop(&mut self) {
        get_isol_table().remove(&self.id).unwrap();
    }
}
