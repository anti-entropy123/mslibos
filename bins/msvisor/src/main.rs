use std::{sync::Arc, thread};

use clap::{arg, Parser};
use derive_more::Display;

use libmsvisor::{
    isolation::{config::IsolationConfig, get_isol, Isolation},
    logger,
};

#[derive(clap::ValueEnum, Clone, Display, Debug)]
pub enum MetricOpt {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "all")]
    All,
    #[display(fmt = "mem")]
    Mem,
}

impl MetricOpt {
    fn to_analyze(&self) -> libmsvisor::MetricOpt {
        match self {
            MetricOpt::None => libmsvisor::MetricOpt::None,
            MetricOpt::All => libmsvisor::MetricOpt::All,
            MetricOpt::Mem => libmsvisor::MetricOpt::Mem,
        }
    }
}

/// A memory-safe LibOS runtime for serverless.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Config file path.
    #[arg(short, long)]
    files: Vec<String>,

    /// Preload all user modules to speed up.
    #[arg(long, default_value_t = false)]
    preload: bool,

    /// show metrics json.
    #[arg(long, default_value_t = MetricOpt::None)]
    metrics: MetricOpt,
}

fn main() {
    logger::init();

    let args = Args::parse();
    let configs: Vec<_> = if !args.files.is_empty() {
        args.files
            .iter()
            .map(|f| IsolationConfig::from_file(f.into()).expect("Open config file failed."))
            .collect()
    } else {
        log::debug!("missing arg files");
        vec![
            IsolationConfig::from_file(String::from("base_config.json").into())
                .expect("Open config file failed."),
        ]
    };

    // info!("preload?:{}", args.preload);
    let isols: Vec<_> = configs.iter().map(Isolation::new).collect();

    if args.preload {
        for (idx, isol) in isols.iter().enumerate() {
            isol.preload(configs.get(idx).unwrap())
                .expect("preload failed.")
        }
    }

    let mut handles: Vec<_> = (1..isols.len() + 1)
        .map(|isol_handle| {
            let builder = thread::Builder::new()
                .name(format!("isol_{}", isol_handle))
                .stack_size(8 * 1024 * 1024);

            let isol_start_with_thread = move || {
                get_isol(isol_handle as u64)
                    .expect("isol don't exist?")
                    .run()
            };

            Some(
                builder
                    .spawn(isol_start_with_thread)
                    .expect("spawn thread failed?"),
            )
        })
        .collect();

    for (idx, handle) in handles.iter_mut().enumerate() {
        let app_result = handle.take().unwrap().join().expect("join failed");
        if app_result.is_err() {
            log::error!("isol{} run failed.", idx)
        }

        let isol = isols.get(idx).unwrap();
        log::info!("isol{} has strong count={}", idx, Arc::strong_count(isol));
        if !matches!(args.metrics, MetricOpt::None) {
            isol.metric.analyze(&args.metrics.to_analyze());
        }
    }
}
