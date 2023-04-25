use std::path::PathBuf;

use msvisor::isolation::config::IsolationConfig;

const TARGET_DIR: &str = ""; //env!("CARGO_MANIFEST_DIR");

fn main() {
    let debug_target_dir = PathBuf::from(TARGET_DIR)
        // .parent()
        // .unwrap()
        .join("target/debug");

    let config1 = IsolationConfig {
        services: Vec::from([("fdtab".to_owned(), debug_target_dir.join("libfdtab.so"))]),
        app: (
            "hello1".to_owned(),
            debug_target_dir.join("libhello_world.so"),
        ),
    };

    config1
        .to_file("./config.json".into())
        .expect("Write to file failed.")
}
