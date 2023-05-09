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
    // Config file path.
    #[arg(short, long, default_value_t = String::from("base_config.json"))]
    file: String,
}

fn main() {
    logger::init();

    let args = Args::parse();

    let config1 = IsolationConfig::from_file(args.file.into()).expect("Open config file failed.");

    let isol1 = Isolation::new(config1);
    isol1.run();
    log::info!("isol1 has strong count={}", Arc::strong_count(&isol1));
    isol1.metric.analyze();
    drop(isol1);
}
