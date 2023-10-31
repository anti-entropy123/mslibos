// use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;

fn main() {
    let read_start = SystemTime::now();
    let content = {
        let mut f = File::open(&format!("../../fs_images/fake_data_{}.txt", 0)).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("read file failed.");
        buf
    };
    let read_finish = SystemTime::now();

    println!(
        "read cost: {} ms",
        read_finish.duration_since(read_start).unwrap().as_millis()
    );

    let mut counter = hashbrown::HashMap::new();

    for line in content.lines() {
        let words = line
            .trim()
            .split(' ')
            .filter(|word| word.chars().all(char::is_alphabetic));

        for word in words {
            let old_count = *counter.entry(word).or_insert(0u32);
            counter.insert(word, old_count + 1);
        }
    }
    let comp_finish = SystemTime::now();

    println!(
        "compute cost: {} ms",
        comp_finish.duration_since(read_finish).unwrap().as_millis()
    );

    println!("reducer has counted {} words", counter.len());
}
