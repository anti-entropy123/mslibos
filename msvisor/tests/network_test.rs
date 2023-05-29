use std::sync::Arc;

use msvisor::isolation::{config::IsolationConfig, Isolation};

#[test]
fn run_send_request_test() {
    use msvisor::logger;
    logger::init();
    if std::env::var("SUDO_PASSWD").is_err() {
        return;
    }

    let config1 =
        IsolationConfig::from_file("simple_http.json".into()).expect("Open config file failed.");

    let isol1 = Isolation::new(config1);
    isol1.run().expect("isolation user function error.");

    log::info!("isol1 has strong count={}", Arc::strong_count(&isol1));
    isol1.metric.analyze();
    drop(isol1);
}
