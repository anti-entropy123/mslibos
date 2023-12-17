use std::{collections::HashMap, time::SystemTime};

use s3::{creds::Credentials, Bucket, Region};
use serde_json::json;

type Error = Box<dyn std::error::Error>;

const MINIO_BASE_URL: &str = "minio-service.openfaas-yjn.svc.cluster.local:9000";
const APP: &str = "parallel_sort";

pub fn handle(body: Vec<u8>) -> Result<Vec<u8>, Error> {
    let event: HashMap<String, serde_json::Value> = serde_json::from_slice(&body)?;
    let input_part = event["input_part"].as_u64().unwrap();
    let input_name = &event["input_name"].as_str().unwrap();
    let sorter_num = event["sorter_num"].as_u64().unwrap() as usize;

    let read_start = SystemTime::now();
    let content = get_object(input_name, &format!("part-{}", input_part)).unwrap();
    let read_end: SystemTime = SystemTime::now();

    let mut sorter_resp: Vec<u32> = Vec::new();
    let mut pivots = Vec::new();

    for num in content.split(',') {
        let num = num.trim();
        if num.is_empty() {
            continue;
        }
        sorter_resp.push(
            num.parse::<u32>()
                .map_err(|_| format!("parse Int failed, num={}", num))?,
        )
    }
    sorter_resp.sort();

    if input_part == 0 {
        // let mut pivots: DataBuffer<VecArg> = ;
        pivots = (0..sorter_num - 1)
            .map(|i| {
                let idx = (i + 1) * sorter_resp.len() / sorter_num;
                sorter_resp[idx]
            })
            .collect();
    }

    let com_end = SystemTime::now();
    put_object_to_minio(
        &format!("part-{}", input_part),
        &sorter_resp
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join(","),
    )
    .unwrap();

    if input_part == 0 {
        put_object_to_minio(
            &"pivots".to_owned(),
            &pivots
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<_>>()
                .join(","),
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

    let bucket_name = "rust-sorter-resp";
    let bucket = {
        Bucket::create_with_path_style(bucket_name, region, credentials, Default::default())?.bucket
    };

    bucket.put_object(object_name, val.as_bytes())?;

    Ok(())
}
