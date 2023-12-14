use std::{collections::HashMap, time::SystemTime};

use s3::{creds::Credentials, Bucket, Region};
use serde_json::json;

type Error = Box<dyn std::error::Error>;

const MINIO_BASE_URL: &str = "minio-service.yasb-mapreduce-db.svc.cluster.local:9000";
const APP: &str = "parallel_sort";

fn parse_to_vec(numbers: String) -> Vec<u32> {
    let mut result = Vec::new();
    for num in numbers.split(',') {
        let num = num.trim();
        if num.is_empty() {
            continue;
        }
        result.push(
            num.parse::<u32>()
                .map_err(|_| format!("parse Int failed, num={}", num))
                .unwrap(),
        )
    }
    result
}

fn dump_to_string(numbers: &[u32]) -> String {
    numbers
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn handle(body: Vec<u8>) -> Result<Vec<u8>, Error> {
    let event: HashMap<String, serde_json::Value> = serde_json::from_slice(&body)?;
    let input_part = event["input_part"].as_u64().unwrap();
    let input_name = &event["input_name"].as_str().unwrap();

    let read_start = SystemTime::now();
    let pivots = get_object(input_name, &"pivots".to_owned())?;
    let numbers = get_object(input_name, &format!("part-{}", input_part))?;
    let read_end: SystemTime = SystemTime::now();

    let pivots = parse_to_vec(pivots);
    let numbers = parse_to_vec(numbers);

    let partitions = split_numbers(&numbers, &pivots);

    let com_end = SystemTime::now();

    for (idx, part) in partitions.iter().enumerate() {
        put_object_to_minio(
            &format!("splitter-{}-resp-part-{}", input_part, idx),
            &dump_to_string(part),
        )
        .unwrap();
    }

    let store_end = SystemTime::now();

    let resp = json!({
        "read_time": read_end.duration_since(read_start).unwrap().as_millis(),
        "comp_time": com_end.duration_since(read_end).unwrap().as_millis(),
        "store_time": store_end.duration_since(com_end).unwrap().as_millis(),
    });

    Ok(resp.to_string().into_bytes())
}

fn get_object(bucket_name: &str, object_name: &String) -> Result<String, Error> {
    let region = Region::Custom {
        region: "none".to_owned(),
        endpoint: format!("http://{}", MINIO_BASE_URL),
    };
    let credentials = Credentials::new(Some("admin123"), Some("admin123"), None, None, None)?;

    let bucket = Bucket::new(bucket_name, region, credentials.clone())?.with_path_style();

    let response_data = bucket.get_object(object_name)?;

    Ok(response_data.as_str()?.to_owned())
}

fn put_object_to_minio(object_name: &String, val: &String) -> Result<(), Error> {
    let region = Region::Custom {
        region: "none".to_owned(),
        endpoint: format!("http://{}", MINIO_BASE_URL),
    };
    let credentials = Credentials::new(Some("admin123"), Some("admin123"), None, None, None)?;

    let bucket_name = "rust-splitter-resp";
    let bucket = {
        Bucket::create_with_path_style(bucket_name, region, credentials, Default::default())?.bucket
    };

    bucket.put_object(object_name, val.as_bytes())?;

    Ok(())
}

fn split_numbers(numbers: &Vec<u32>, pivots: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    let mut current_start = 0;

    for &pivot in pivots {
        let mut current_partition = Vec::new();

        for &num in &numbers[current_start..] {
            if num < pivot {
                current_partition.push(num);
            } else {
                break;
            }
        }

        result.push(current_partition.clone());
        current_start += current_partition.len();

        if current_start >= numbers.len() {
            break;
        }
    }

    // Add the remaining numbers as the last partition
    result.push(numbers[current_start..].to_vec());

    result
}
