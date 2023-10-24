use std::{path::PathBuf, sync::Arc};

use libmsvisor::{
    isolation::{config::IsolationConfig, Isolation},
    logger,
};

#[test]
fn run_operate_file_test() {
    logger::init();

    if !PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("fs_images")
        .join("fatfs.img")
        .is_file()
    {
        log::warn!("fs image is not exists. ignore file_test");
        return;
    }

    let config1 =
        IsolationConfig::from_file("simple_file.json".into()).expect("Open config file failed.");

    let isol1 = Isolation::new(&config1);
    isol1.run().expect("isolation user function error.");

    log::info!("isol1 has strong count={}", Arc::strong_count(&isol1));
    isol1.metric.analyze(&libmsvisor::MetricOpt::None);
    drop(isol1);
}
