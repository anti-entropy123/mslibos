use msvisor::{isolation::config::IsolationConfig, utils};

fn main() {
    let debug_target_dir = &utils::TARGET_DEBUG_PATH;

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
