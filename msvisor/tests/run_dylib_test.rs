use std::{path::PathBuf, thread};

use msvisor::{
    isolation::{Isolation, IsolationConfig},
    logger,
};

const TARGET_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../target");

#[test]
fn run_dylib_test() {
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
}

#[test]
fn run_multi_dylib_test() {
    for _ in 0..30 {
        thread::spawn(|| run_dylib_test())
            .join()
            .expect("join thread failed");
    }
}
