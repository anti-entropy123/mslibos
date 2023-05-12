use msvisor::{
    isolation::{config::IsolationConfig, Isolation},
    utils,
};

#[test]
fn run_dylib_test() {
    // logger::init();

    let debug_target_dir = &utils::TARGET_DEBUG_PATH;

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
    assert!(isol1.run().is_ok());
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
