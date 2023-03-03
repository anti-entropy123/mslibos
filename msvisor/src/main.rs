use msvisor::{isolation::Isolation, logger};

fn main() {
    logger::init();

    let isol1 = Isolation::new();
    let isol2 = Isolation::new();
    isol1.run();
    isol2.run();
}
