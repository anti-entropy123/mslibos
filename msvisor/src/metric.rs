use std::sync::{Arc, Mutex};

use ms_hostcall::types::ServiceName;
use serde::Serialize;
use serde_json::{json, Value};

use crate::now_millis;

pub enum MetricEvent {
    // IsolationEvent
    LoadService,

    // SvcEvent
    SvcInit,
    SvcRun,
    SvcEnd,
}

#[derive(Default, Serialize)]
struct MetricBucketInner {
    #[serde(skip)]
    svc_metrics: Vec<Arc<SvcMetricBucket>>,

    load_service_num: u32,
}

impl MetricBucketInner {
    fn to_json(&self) -> Value {
        serde_json::json!(self)
    }
}

pub struct MetricBucket {
    inner: Mutex<MetricBucketInner>,
}

impl Default for MetricBucket {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricBucket {
    pub fn new() -> Self {
        MetricBucket {
            inner: Mutex::new(Default::default()),
        }
    }

    pub fn new_svc_metric(&self, name: ServiceName) -> Arc<SvcMetricBucket> {
        let svc_metric = Arc::new(SvcMetricBucket::new(name));
        let mut inner = self.inner.lock().unwrap();
        inner.svc_metrics.push(Arc::clone(&svc_metric));

        svc_metric
    }

    pub fn mark(&self, event: MetricEvent) {
        let mut inner = self.inner.lock().unwrap();
        match event {
            MetricEvent::LoadService => inner.load_service_num += 1,

            _ => {
                panic!("error metric event for isolation MetricBucket")
            }
        }
    }

    pub fn analyze(&self) {
        let inner = self.inner.lock().unwrap();
        let mut result = serde_json::Value::default();

        result["isolation"] = inner.to_json();
        result["services"] = {
            let svc_metrics: Vec<Value> = inner
                .svc_metrics
                .iter()
                .map(|metric| metric.to_json())
                .collect();
            serde_json::json!(svc_metrics)
        };

        println!("{}", result)
    }
}

#[derive(Default, Serialize)]
struct SvcMetricBucketInner {
    init_t: u128,
    run_t: u128,
    end_t: u128,
}

impl SvcMetricBucketInner {
    fn to_json(&self) -> Value {
        let mut val = json!({ "raw": self });

        if self.end_t > 0 {
            val["run_time(ms)"] = json!(self.end_t - self.run_t);
            val["init_time(ms)"] = json!(self.end_t - self.init_t);
        };

        val
    }
}

pub struct SvcMetricBucket {
    svc_name: ServiceName,

    inner: Mutex<SvcMetricBucketInner>,
}

impl SvcMetricBucket {
    fn new(name: ServiceName) -> Self {
        SvcMetricBucket {
            inner: Default::default(),
            svc_name: name,
        }
    }

    pub fn mark(&self, event: MetricEvent) {
        let mut inner = self.inner.lock().unwrap();
        match event {
            MetricEvent::SvcInit => inner.init_t = now_millis!(),
            MetricEvent::SvcRun => inner.run_t = now_millis!(),
            MetricEvent::SvcEnd => inner.end_t = now_millis!(),

            _ => {
                panic!("error metric event for Service MetricBucket")
            }
        }
    }

    pub fn to_json(&self) -> Value {
        log::debug!("analyze service_{} metrics.", self.svc_name);
        json!({ self.svc_name.clone():  self.inner.lock().unwrap().to_json()})
    }
}
