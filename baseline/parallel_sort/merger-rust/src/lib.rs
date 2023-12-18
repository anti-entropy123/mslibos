use std::{collections::HashMap, time::SystemTime};

use s3::{creds::Credentials, Bucket, Region};
use serde_json::json;

type Error = Box<dyn std::error::Error>;

const MINIO_BASE_URL: &str = "minio-service.openfaas-yjn.svc.cluster.local:9000";
const APP: &str = "parallel_sort";

fn parse_to_vec(numbers: &str) -> Vec<u32> {
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
    let input_name = event["input_name"].as_str().unwrap();
    let sorter_num = event["sorter_num"].as_u64().unwrap();

    let read_start = SystemTime::now();
    let partitions: Vec<String> = (0..sorter_num)
        .map(|idx| {
            get_object(
                input_name,
                &format!("splitter-{}-resp-part-{}", idx, input_part),
            )
            .unwrap()
        })
        .collect();

    let read_end: SystemTime = SystemTime::now();

    let partitions: Vec<Vec<_>> = partitions.iter().map(|part| parse_to_vec(part)).collect();
    let merge_result = merge_partitions(partitions);

    let com_end = SystemTime::now();

    put_object_to_minio(
        &format!("merge_result_{}", input_part),
        &dump_to_string(&merge_result),
    )
    .unwrap();

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

    let bucket_name = "rust-merger-resp";
    let bucket = {
        Bucket::create_with_path_style(bucket_name, region, credentials, Default::default())?.bucket
    };

    bucket.put_object(object_name, val.as_bytes())?;

    Ok(())
}

fn merge_partitions(partitions: Vec<Vec<u32>>) -> Vec<u32> {
    let mut result = Vec::new();

    let mut indices: Vec<usize> = partitions.iter().map(|_| 0).collect();

    loop {
        let mut min_value = core::u32::MAX;
        let mut min_partition = None;

        for (i, &index) in indices.iter().enumerate() {
            if index < partitions[i].len()
                && *partitions[i].get(index).unwrap_or(&u32::MAX) < min_value
            {
                min_value = partitions[i][index];
                min_partition = Some(i);
            }
        }

        match min_partition {
            Some(partition_idx) => {
                result.push(min_value);
                indices[partition_idx] += 1;
            }
            None => break,
        }
    }

    result
}
