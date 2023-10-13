use libmsvisor::isolation::{config::IsolationConfig, Isolation};

// #[cfg(not(feature = "namespace"))]
#[test]
fn long_chain_test() {
    use libmsvisor::logger;

    logger::init();

    let config1 =
        IsolationConfig::from_file("long_chain.json".into()).expect("Open config file failed.");

    let isol1 = Isolation::new(&config1);
    isol1.preload(&config1).expect("preload failed");

    assert!(isol1.run().is_ok());
}
