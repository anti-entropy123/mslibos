use std::{path::PathBuf, sync::Arc};

use msvisor::{
    isolation::{Isolation, IsolationConfig},
    logger,
};

const TARGET_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../target");

fn main() {
    logger::init();

    let debug_target_dir = PathBuf::from(TARGET_DIR).join("debug");

    let config1 = IsolationConfig {
        services: Vec::from([("fs".to_owned(), debug_target_dir.join("libnative_fs.so"))]),
        app: (
            "hello1".to_owned(),
            debug_target_dir.join("libhello_world.so"),
        ),
    };

    let isol1 = Isolation::new(config1);
    isol1.run();
    log::info!("isol1 has strong count={}", Arc::strong_count(&isol1));
    drop(isol1);

    let config2 = IsolationConfig {
        services: Vec::from([("fs".to_owned(), debug_target_dir.join("libnative_fs.so"))]),
        app: (
            "hello2".to_owned(),
            debug_target_dir.join("libhello_world.so"),
        ),
    };

    let isol2 = Isolation::new(config2);
    isol2.run();
    log::info!("isol2 has strong count={}", Arc::strong_count(&isol2));
    drop(isol2);
}
