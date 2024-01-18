use std::{collections::HashMap, sync::OnceLock};

use axum::extract::{Path, Query};
use memcache::{self, Client};

const MEMCACHE_URL: &str = "memcache://localhost:11211";
// const MEMCACHE_URL: &str = "memcache://user-memcached.movie-reviewing.svc.cluster.local:11211";

static CLIENT: OnceLock<Client> = OnceLock::new();
fn get_client() -> &'static Client {
    CLIENT.get_or_init(|| memcache::connect(MEMCACHE_URL).expect("client connect failed."))
}

pub async fn get(Path(key): Path<String>) -> String {
    log::info!("memcache::get, key={}", key);
    let client = get_client();
    // client.flush().unwrap();

    let result: Option<String> = client.get(&key).unwrap();
    result.unwrap_or_default()
}

pub async fn set(Query(params): Query<HashMap<String, String>>) {
    let client: &Client = get_client();
    let key = params.get("key").unwrap();
    let value = params.get("value").unwrap();
    log::info!("memcache::set, key={}, value={}", key, value);

    client.set(key, value, 120).unwrap();
    // client.get(key).unwrap().unwrap()
}
