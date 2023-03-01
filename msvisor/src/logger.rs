pub use log::{debug, error, info, warn};
use std::env;

pub fn init() {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "DEBUG");
    }
    env_logger::init();
}
