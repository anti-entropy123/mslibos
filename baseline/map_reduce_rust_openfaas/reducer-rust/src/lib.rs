use std::{collections::HashMap, time::SystemTime};

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
    let reduce_part = &event["reduce_part"].as_u64().unwrap();
    let input_num = &event["input_num"].as_u64().unwrap();

    let read_start = SystemTime::now();

    let mut raw_datas: Vec<String> = Vec::with_capacity(*input_num as usize);
    for i in 0..*input_num {
        raw_datas.push(
            get_object_from_redis(&format!(
                "rust-{}:{}:{}:{}",
                input_name, APP, i, reduce_part
            ))
            .map_err(|e| format!("get_object_from_redis: {}", e))?,
        )
    }

    let read_end = SystemTime::now();

    let mut counter: hashbrown::HashMap<String, u32> = hashbrown::HashMap::new();
    for raw_data in raw_datas {
        let pairs = raw_data.split(';').map(|s| s.split(':'));
        for mut pair in pairs {
            match (pair.next(), pair.next(), pair.next()) {
                (Some(word), Some(count), None) => {
                    let old_val = counter.get(word).copied().unwrap_or(0);
                    counter.insert(word.to_owned(), old_val + count.parse::<u32>()?);
                }
                _ => Err(format!("wrong redis input object: {:?}", pair))?,
            }
        }
    }

    let output_entries: Vec<_> = counter
        .iter_mut()
        .map(|(word, count)| format!("{}:{}", word, count))
        .collect();
    let output = output_entries.join("\n");
    let comp_end = SystemTime::now();

    put_object_to_minio(&format!("part-{}", reduce_part), &output)
        .map_err(|e| format!("put_object_to_minio: {}", e))?;
    let store_end = SystemTime::now();

    let resp = json!({
        "read_time": read_end.duration_since(read_start).unwrap().as_millis(),
        "comp_time": comp_end.duration_since(read_end).unwrap().as_millis(),
        "store_time": store_end.duration_since(comp_end).unwrap().as_millis(),
    });
    Ok(resp.to_string().into_bytes())
}

fn put_object_to_minio(object_name: &String, val: &String) -> Result<(), Error> {
    let region = Region::Custom {
        region: "none".to_owned(),
        endpoint: format!("http://{}", MINIO_BASE_URL),
    };
    let credentials = Credentials::new(Some("admin123"), Some("admin123"), None, None, None)?;

    let bucket_name = &format!("rust-{}-output", APP);
    let bucket = {
        // let b = Bucket::new(bucket_name, region.clone(), credentials.clone())?.with_path_style();

        // if b.exists()? {
        //     b
        // } else {
        Bucket::create_with_path_style(bucket_name, region, credentials, Default::default())?.bucket
        // }
    };

    // let old_header = bucket.head_object(object_name);
    // if old_header.is_ok() {
    //     bucket.delete_object(object_name)?;
    // }

    bucket.put_object(object_name, val.as_bytes())?;

    Ok(())
}

fn get_object_from_redis(key: &String) -> Result<String, Error> {
    let client = redis::Client::open(format!("redis://{}/", REDIS_BASE_URL))?;
    let mut con = client.get_connection()?;
    Ok(con.get(key)?)
}
