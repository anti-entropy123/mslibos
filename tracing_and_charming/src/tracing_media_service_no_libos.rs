use std::{thread::sleep, time::Duration};

use tracing_and_charming::run_media_service;

fn main() {
    let mut media_service_no_libos = Vec::with_capacity(100);
    for _ in 0..100 {
        media_service_no_libos.push(run_media_service(false));
        sleep(Duration::from_millis(100))
    }
    println!("{:?}", &media_service_no_libos[10..])
}
