use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // fn get(Path(key): Path<String>) -> String
        .route("/memcache/get/:key", get(db_agent::memcache::get))
        // fn set(Query(params): Query<HashMap<String, String>>)
        .route("/memcache/set/", post(db_agent::memcache::set))
        // fn find(
        //     Path(db_name): Path<String>,
        //     Path(col_name): Path<String>,
        //     Query(find_args): Query<HashMap<String, String>>,
        // ) -> String {
        .route(
            "/mongodb/find/:db_name/:col_name",
            get(db_agent::mongodb::find),
        )
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
