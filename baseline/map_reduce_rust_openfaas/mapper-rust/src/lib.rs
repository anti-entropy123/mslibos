use std::time::SystemTime;

use minio::s3::{args::GetObjectArgs, client::Client, creds::StaticProvider};

type Error = Box<dyn std::error::Error>;

const PHRASE: &str = "Hello, World!";
const MINIO_BASE_URL: &str = "minio-service.yasb-mapreduce-db.svc.cluster.local:9000";

pub fn handle(_body: Vec<u8>) -> Result<Vec<u8>, Error> {
    let read_start = SystemTime::now();

    let minio_provider = StaticProvider::new("admin123", "admin123", None);
    let minio_client = Client::new(
        MINIO_BASE_URL.parse()?,
        Some(Box::new(minio_provider)),
        None,
        None,
    )?;

    let content = {
        let obj_name = format!("part-{}", 0);
        let get_object_arg = GetObjectArgs::new("data-500m", &obj_name)?;
        let input_data = minio_client.get_object(&get_object_arg);
        let runtime = tokio::runtime::Builder::new_current_thread().build()?;
        let input_data = runtime.block_on(input_data)?;
        runtime.block_on(input_data.text())?
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

    Ok(PHRASE.as_bytes().to_vec())
}
