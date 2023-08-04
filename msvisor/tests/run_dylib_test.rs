use msvisor::isolation::{config::IsolationConfig, Isolation};

#[test]
fn run_dylib_test() {
    // logger::init();
    let config1 =
        IsolationConfig::from_file("base_config.json".into()).expect("Open config file failed.");

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
