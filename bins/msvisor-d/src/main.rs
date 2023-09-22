use axum::{extract::Query, response::IntoResponse, routing::get, Router};
use libmsvisor::{
    isolation::{config::IsolationConfig, Isolation},
    logger,
};
use serde::Deserialize;

type AppResult<T> = Result<T, AppError>;
struct AppError(String);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        self.0.into_response()
    }
}
impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        AppError(value.to_string())
    }
}

#[derive(Deserialize)]
struct TrigeWorkflowReq {
    isol_name: String,
}

async fn trige_workflow_handler(
    Query(TrigeWorkflowReq { mut isol_name }): Query<TrigeWorkflowReq>,
) -> AppResult<String> {
    if !isol_name.ends_with(".json") {
        isol_name += ".json"
    };
    let config = IsolationConfig::from_file(isol_name.into())?;

    // info!("preload?:{}", args.preload);
    let isol = Isolation::new(config.clone());
    if isol.run().is_err() {
        logger::error!("isolation user function error.")
    }

    Ok("ok".to_owned())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    logger::init();
    let app = Router::new().route("/workflow", get(trige_workflow_handler));

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
