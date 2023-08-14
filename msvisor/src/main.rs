use std::sync::Arc;

use clap::{arg, Parser};

use msvisor::{
    isolation::{config::IsolationConfig, Isolation},
    logger,
};

/// A memory-safe LibOS runtime for serverless.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Config file path.
    #[arg(short, long, default_value_t = String::from("base_config.json"))]
    file: String,

    /// Preload all user modules to speed up.
    #[arg(long, default_value_t = false)]
    preload: bool,
}

fn main() {
    logger::init();

    let args = Args::parse();

    let config1 = IsolationConfig::from_file(args.file.into()).expect("Open config file failed.");

    // info!("preload?:{}", args.preload);
    let isol1 = Isolation::new(config1.clone());
    if args.preload {
        isol1.preload(config1)
    }

    if isol1.run().is_err() {
        logger::error!("isolation user function error.")
    }

    log::info!("isol1 has strong count={}", Arc::strong_count(&isol1));
    isol1.metric.analyze();
    drop(isol1);
}
