use std::time::SystemTime;

use s3::{creds::Credentials, Bucket, Region};

type Error = Box<dyn std::error::Error>;

const MINIO_BASE_URL: &str = "minio-service.yasb-mapreduce-db.svc.cluster.local:9000";
const BUCKET_NAME: &str = "data-500m";
const OBJ_NAME: &str = "part-0";

pub fn handle(_body: Vec<u8>) -> Result<Vec<u8>, Error> {
    let read_start = SystemTime::now();

    let content = {
        let region = Region::Custom {
            region: "none".to_owned(),
            endpoint: format!("http://{}", MINIO_BASE_URL),
        };
        let credentials = Credentials::new(Some("amdin123"), Some("amdin123"), None, None, None)?;

        let bucket =
            Bucket::new(BUCKET_NAME, region.clone(), credentials.clone())?.with_path_style();

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let response_data = runtime.block_on(bucket.get_object(OBJ_NAME))?;

        response_data.as_str()?.to_owned()
    };
    let read_finish: SystemTime = SystemTime::now();

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

    Ok(format!("reducer has counted {} words", counter.len())
        .as_bytes()
        .to_vec())
}
