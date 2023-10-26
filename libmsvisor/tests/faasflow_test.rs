use libmsvisor::isolation::{config::IsolationConfig, Isolation};

#[test]
fn faasflow_test() {
    // logger::init();
    // let config1 =
    //     IsolationConfig::from_file("multi_apps.json".into()).expect("Open config file failed.");

    // let isol1 = Isolation::new(config1);
    // assert!(isol1.run().is_ok());
    // drop(isol1);

    let config2 = IsolationConfig::from_file("pass_complex_args.json".into())
        .expect("Open config file failed.");

    let isol2 = Isolation::new(&config2);

    isol2.preload(&config2).expect("preload failed");

    assert!(isol2.run().is_ok());
}
