use libmsvisor::{
    isolation::{config::IsolationConfig, Isolation},
    logger,
};

#[test]
fn run_should_panic_test() {
    logger::init();

    let config1 =
        IsolationConfig::from_file("should_panic.json".into()).expect("Open config file failed.");

    let isol1 = Isolation::new(&config1);
    assert!(isol1.run().is_err());
}
