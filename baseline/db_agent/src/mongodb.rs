use std::{collections::HashMap, sync::OnceLock};

use axum::{
    extract::{Path, Query},
    Json,
};
use mongodb::{bson::Document, options::ClientOptions, Client, Cursor};
use serde_json::json;

const MONGO_URL: &str = "mongodb://user-mongodb.movie-reviewing.svc.cluster.local:27017/";

static CLIENT: OnceLock<Client> = OnceLock::new();
fn get_client() -> Client {
    CLIENT
        .get_or_init(|| {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .build()
                .unwrap();
            let client_options = runtime.block_on(ClientOptions::parse(MONGO_URL)).unwrap();
            Client::with_options(client_options).unwrap()
        })
        .clone()
}

pub async fn find(
    Path(db_name): Path<String>,
    Path(col_name): Path<String>,
    Query(find_args): Query<HashMap<String, String>>,
) -> String {
    let client = get_client();
    let db = client.database(&db_name);
    let col: mongodb::Collection<Document> = db.collection(&col_name);
    let mut filter = Document::new();
    for (k, v) in find_args.iter() {
        filter.insert(k, v);
    }

    let mut cursor = col.find(Some(filter), None).await.unwrap();
    cursor
        .advance()
        .await
        .map(|doc| {
            if doc {
                json!(cursor.current()).to_string()
            } else {
                String::new()
            }
        })
        .unwrap()
}

pub async fn insert(
    Path(db_name): Path<String>,
    Path(col_name): Path<String>,
    Query(mut params): Query<HashMap<String, String>>,
) {
    let client = get_client();
    let db = client.database(&db_name);
    let col: mongodb::Collection<Document> = db.collection(&col_name);

    let doc_body: &mut String = params.get_mut("doc").unwrap();
    let buffer = unsafe { doc_body.as_mut_vec() };

    // col.insert_one(
    //     Document::from(std::io::Cursor::new(buffer.clone())).unwrap(),
    //     None,
    // )
    // .await
    // .unwrap();
}
