use reqwest::Result;

async fn mslibos_client() {
    const URL: &str = "http://192.168.1.141:8000/workflow?isol_name=never_stop";
    reqwest::get(URL);
}

#[tokio::main]
async fn main() {
    mslibos_client().await;
    println!("req1",);
    mslibos_client().await;
    println!("req2",);
}
