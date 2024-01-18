use std::{thread::sleep, time::Duration};

use tracing_and_charming::run_media_service;

fn main() {
    let mut media_service = Vec::with_capacity(100);
    for _ in 0..100 {
        media_service.push(run_media_service(true));
        sleep(Duration::from_millis(100))
    }
    println!("{:?}", &media_service[10..]);
}
