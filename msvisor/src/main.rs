use std::sync::Arc;

use msvisor::{isolation::Isolation, logger};

fn main() {
    logger::init();
    {
        let isol1 = Isolation::new();
        isol1.run();
        log::info!("isol1 has strong count={}", Arc::strong_count(&isol1))
    }
    {
        let isol2 = Isolation::new();
        isol2.run();
    }
}
