use std::path::PathBuf;

use msvisor::{logger, service::Service};

fn main() {
    logger::init();

    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target")
        .join("debug")
        .join("libhello_world.so");

    Service::new("hello1", filename).run();
}
