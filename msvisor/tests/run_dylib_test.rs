use std::path::PathBuf;

use msvisor::isolation::{config::IsolationConfig, Isolation};

const TARGET_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[test]
fn run_dylib_test() {
    // logger::init();

    let debug_target_dir = PathBuf::from(TARGET_DIR)
        .parent()
        .unwrap()
        .join("target/debug");

    let config1 = IsolationConfig {
        services: Vec::from([
            ("fdtab".to_owned(), debug_target_dir.join("libfdtab.so")),
            ("stdio".to_owned(), debug_target_dir.join("libstdio.so")),
        ]),
        app: (
            "hello1".to_owned(),
            debug_target_dir.join("libhello_world.so"),
        ),
    };

    let isol1 = Isolation::new(config1);
    isol1.run();
}

// This test have stack overflow error.
// #[test]
// fn run_multi_dylib_test() {
//     for _ in 0..5 {
//         thread::spawn(|| run_dylib_test())
//             .join()
//             .expect("join thread failed");
//     }
// }

// This test doesn't work because panic_handler haven't been
// implemented correctly.
// #[test]
// #[should_panic]
// fn run_should_panic_test() {
//     logger::init();

//     let debug_target_dir = PathBuf::from(TARGET_DIR).join("debug");

//     let config1 = IsolationConfig {
//         services: Vec::new(),
//         app: (
//             "should_panic".to_owned(),
//             debug_target_dir.join("libshould_panic.so"),
//         ),
//     };

//     let isol1 = Isolation::new(config1);
//     isol1.run();
// }
