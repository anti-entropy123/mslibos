use std::{
    fs,
    sync::{Arc, Mutex},
    time::UNIX_EPOCH,
};

use ms_hostcall::types::{MetricEvent, ServiceName};
use serde::Serialize;
use serde_json::{json, Value};

use crate::now_millis;

#[derive(Default, Serialize)]
struct MetricBucketInner {
    #[serde(skip)]
    svc_metrics: Vec<Arc<SvcMetricBucket>>,

    load_service_num: u32,
    mem_metrics: Vec<(u128, usize)>,
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
            MetricEvent::Mem => {
                let timestamp = std::time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_micros();
                let vm_rss_kb = get_current_vm_rss_kb();

                inner.mem_metrics.push((timestamp, vm_rss_kb))
            }

            _ => {
                panic!(
                    "error metric event for isolation MetricBucket, event={:?}",
                    event
                )
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

        eprintln!(
            "{}",
            serde_json::to_string_pretty(&result).expect("format json failed")
        );
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

fn get_current_vm_rss_kb() -> usize {
    let proc_status = fs::read_to_string("/proc/self/status").expect("get proc status failed.");

    let line = proc_status
        .lines()
        .find(|line| line.starts_with("VmRSS:"))
        .expect("wrong status content");

    // assert!(line.len() == 1, "{}", line);

    let number_vec: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();

    assert!(!number_vec.is_empty());

    let numbers = String::from_iter(number_vec);
    assert!(!numbers.is_empty());

    numbers
        .parse()
        .unwrap_or_else(|_| panic!("wrong number string, numbers={}", numbers))
}

#[test]
fn get_current_vm_rss_test() {
    let mut data = [0usize; 1024]; // 8 kb
    for i in 0..data.len() {
        data[i] = i
    }

    let vm_rss = get_current_vm_rss_kb();
    assert!(vm_rss > 8, "{}", vm_rss)
}
