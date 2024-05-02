use std::sync::Arc;

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
    #[display(fmt = "total_dur")]
    TotalDur,
    #[display(fmt = "mem")]
    Mem,
}

impl MetricOpt {
    fn to_analyze(&self) -> libmsvisor::MetricOpt {
        match self {
            MetricOpt::None => libmsvisor::MetricOpt::None,
            MetricOpt::All => libmsvisor::MetricOpt::All,
            MetricOpt::Mem => libmsvisor::MetricOpt::Mem,
            MetricOpt::TotalDur => libmsvisor::MetricOpt::TotalDur,
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

#[tokio::main]
async fn main() {
    logger::init();

    let args = Args::parse();
    let configs: Vec<_> = if !args.files.is_empty() {
        args.files
            .iter()
            .map(|f| {
                IsolationConfig::from_file(f.into()).unwrap_or_else(|e| {
                    panic!("Read isol_config file failed, file={}, err={}", f, e)
                })
            })
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
    let isol_handles = isols.iter().map(|isol| isol.id);

    if args.preload {
        for (idx, isol) in isols.iter().enumerate() {
            isol.preload(configs.get(idx).unwrap())
                .expect("preload failed.")
        }
    }

    let mut join_handles: Vec<_> = isol_handles
        .map(|isol_handle| {
            let isol_start_with_thread = move || {
                let isol = get_isol(isol_handle).expect("isol don't exist?");
                let app_result = isol.run();
                if let Err(e) = app_result {
                    log::error!("isol{} run failed. err={e:?}", isol.id)
                }
            };

            Some(tokio::task::spawn_blocking(move || {
                isol_start_with_thread()
            }))
        })
        .collect();

    for (isol_idx, join_handle) in join_handles.iter_mut().enumerate() {
        let join_handle = join_handle.take().unwrap();
        join_handle.await.expect("join failed");

        let isol = isols.get(isol_idx).unwrap();

        log::debug!(
            "isol{} has strong count={}",
            isol.id,
            Arc::strong_count(isol)
        );

        if !matches!(args.metrics, MetricOpt::None) {
            isol.metric.analyze(&args.metrics.to_analyze());
        }
    }
}
