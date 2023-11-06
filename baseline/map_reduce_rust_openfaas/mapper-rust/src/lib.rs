use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash, Hasher},
    time::SystemTime,
};

use redis::Commands;
use s3::{creds::Credentials, Bucket, Region};
use serde_json::json;

type Error = Box<dyn std::error::Error>;

const MINIO_BASE_URL: &str = "minio-service.yasb-mapreduce-db.svc.cluster.local:9000";
const REDIS_BASE_URL: &str = "redis.yasb-mapreduce-db.svc.cluster.local:6379";
const APP: &str = "wc";

pub fn handle(body: Vec<u8>) -> Result<Vec<u8>, Error> {
    let event: HashMap<String, serde_json::Value> = serde_json::from_slice(&body)?;

    let input_name = &event["input_name"].as_str().unwrap();
    let input_part = &event["input_part"].as_u64().unwrap();
    let reduce_num = &event["reduce_num"].as_u64().unwrap();

    let read_start = SystemTime::now();

    let content = get_object(input_name, &format!("part-{}", input_part))?;
    let read_end: SystemTime = SystemTime::now();

    println!(
        "read cost: {} ms",
        read_end.duration_since(read_start).unwrap().as_millis()
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

    let shuffle = shuffle_counter(*reduce_num, &counter);

    let com_end = SystemTime::now();
    println!(
        "compute cost: {} ms",
        com_end.duration_since(read_end).unwrap().as_millis()
    );

    for i in 0..*reduce_num {
        if let Some(val) = shuffle.get(&i) {
            put_object(
                &format!("rust-{}:{}:{}:{}", input_name, APP, input_part, i),
                val,
            )?
        }
    }
    let store_end = SystemTime::now();

    let resp = json!({
        "read_time": read_end.duration_since(read_start).unwrap().as_millis(),
        "comp_time": com_end.duration_since(read_end).unwrap().as_millis(),
        "store_time": store_end.duration_since(com_end).unwrap().as_millis(),
        "count_num": counter.len(),
    });
    Ok(resp.to_string().into_bytes())
}

fn get_object(bucket_name: &str, object_name: &String) -> Result<String, Error> {
    let region = Region::Custom {
        region: "none".to_owned(),
        endpoint: format!("http://{}", MINIO_BASE_URL),
    };
    let credentials = Credentials::new(Some("amdin123"), Some("amdin123"), None, None, None)?;

    let bucket = Bucket::new(bucket_name, region.clone(), credentials.clone())?.with_path_style();

    let response_data = bucket.get_object(object_name)?;

    Ok(response_data.as_str()?.to_owned())
}

fn put_object(key: &String, val: &String) -> Result<(), Error> {
    let client = redis::Client::open(format!("redis://{}/", REDIS_BASE_URL))?;
    let mut con = client.get_connection()?;
    con.set(key, val)?;

    Ok(())
}

fn shuffle_counter(
    reducer_num: u64,
    counter: &hashbrown::HashMap<&str, u32>,
) -> HashMap<u64, String> {
    let mut shuffle: HashMap<u64, String> = HashMap::with_capacity((reducer_num + 5) as usize);

    for (word, count) in counter {
        let mut hasher = hashbrown::hash_map::DefaultHashBuilder::default().build_hasher();
        word.hash(&mut hasher);
        let reduce_id = hasher.finish() % reducer_num;
        let old_val = shuffle
            .get(&reduce_id)
            .map(|s| s.as_str())
            .unwrap_or_else(|| "");

        shuffle.insert(reduce_id, format!("{}{}:{};", old_val, word, count));
    }

    for val in shuffle.values_mut() {
        let s = val.char_indices();
        let (idx, c) = s.last().unwrap();
        if c == ';' {
            *val = val.as_str()[0..idx].to_string();
        }
    }

    shuffle
}

#[test]
fn test_shuffle_counter() {
    let mut input: hashbrown::HashMap<&str, u32> = hashbrown::HashMap::new();
    input.insert("yjn", 3);
    input.insert("abc", 2);
    input.insert("rust", 10);
    input.insert("libos", 7);

    let result = shuffle_counter(2, &input);
    println!("{:?}", result)
}
