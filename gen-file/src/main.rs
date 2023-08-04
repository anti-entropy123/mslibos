use std::path::PathBuf;

use msvisor::isolation::config::IsolationConfig;

fn main() {
    let config1 = IsolationConfig {
        services: vec![
            (
                "fdtab".to_owned(),
                PathBuf::from("target/debug/libfdtab.so"),
            ),
            (
                "stdio".to_owned(),
                PathBuf::from("target/debug/libstdio.so"),
            ),
        ],
        apps: vec![(
            "hello1".to_owned(),
            PathBuf::from("target/debug/libhello_world.so"),
        )],
    };

    config1
        .to_file("./config.json".into())
        .expect("Write to file failed.")
}
