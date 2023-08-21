use std::sync::Arc;

use msvisor::isolation::{config::IsolationConfig, Isolation};

#[test]
fn run_operate_file_test() {
    use msvisor::logger;
    logger::init();

    let config1 =
        IsolationConfig::from_file("simple_file.json".into()).expect("Open config file failed.");

    let isol1 = Isolation::new(config1);
    isol1.run().expect("isolation user function error.");

    log::info!("isol1 has strong count={}", Arc::strong_count(&isol1));
    isol1.metric.analyze();
    drop(isol1);
}
