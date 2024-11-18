use std::sync::Arc;
use tokio;

pub mod memory_pipe;
pub mod runner;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

static PORT: &str = "3000";

struct AppState {
    runner: runner::PythonRunner,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("starting server");

    let runner = runner::PythonRunner::new().await.unwrap();
    let shared_state = Arc::new(AppState { runner });

    let app = Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/run", post(run))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .unwrap();
    info!("server running on port {} 🚀", PORT);
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "ok"
}

#[derive(Deserialize)]
struct RunPayload {
    code: String,
}

#[derive(Serialize, Debug)]
struct Response {
    res: runner::PythonResponse,
}

async fn run(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RunPayload>,
) -> (StatusCode, Json<Response>) {

    let id = Uuid::new_v4();
    let res = state.runner.run_code(&payload.code, true, id).await;
    let msg = Response { res: res.unwrap() };

    (StatusCode::OK, Json(msg))
}
