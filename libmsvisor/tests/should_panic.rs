use std::{env::set_current_dir, path::PathBuf};

use libmsvisor::{
    isolation::{config::IsolationConfig, Isolation},
    logger,
};

const ROOT_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[test]
fn run_should_panic_test() {
    logger::init();

    set_current_dir::<PathBuf>(PathBuf::from(ROOT_DIR).parent().unwrap().into())
        .expect("set current work path failed.");

    println!("{}", std::env::current_dir().unwrap().display());
    let config1 = IsolationConfig::from_file("isol_config/should_panic.json".into())
        .expect("Open config file failed.");

    let isol1 = Isolation::new(&config1);
    assert!(isol1.run().is_ok());
}
