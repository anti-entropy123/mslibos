use std::path::PathBuf;

use msvisor::isolation::config::IsolationConfig;

fn main() {
    let config1 = IsolationConfig {
        services: Vec::from([(
            "fdtab".to_owned(),
            PathBuf::from("target/debug/libfdtab.so"),
        )]),
        app: (
            "hello1".to_owned(),
            PathBuf::from("target/debug/libhello_world.so"),
        ),
    };

    config1
        .to_file("./config.json".into())
        .expect("Write to file failed.")
}
