use std::sync::Arc;

use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};

mod store;
mod util;

use store::{LocalStore, RemoteStore, Store};

#[derive(Deserialize)]
struct WriteRequest {
    data: Vec<u8>, // Serialized Arrow RecordBatch
    path: String,
}

#[derive(Serialize)]
struct WriteResponse {
    status: String,
}

#[derive(Clone)]
struct AppState {
    store: Arc<Box<dyn Store>>,
}

async fn write_handler(
    State(state): State<AppState>,
    Json(req): Json<WriteRequest>,
) -> Result<Json<WriteResponse>, StatusCode> {
    let data = util::read_record_batch_from_vec(req.data).map_err(|_| StatusCode::BAD_REQUEST)?;

    let store = state.store;
    store
        .write(data, &req.path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    store
        .notify_catalog(&req.path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(WriteResponse {
        status: "success".to_string(),
    }))
}

#[tokio::main]
async fn main() {
    let store: Box<dyn Store> = if cfg!(debug_assertions) {
        Box::new(LocalStore {
            base_path: "./data".to_string(),
        })
    } else {
        Box::new(RemoteStore {})
    };

    let store = Arc::new(store);

    let app_state = AppState { store };


    let app = Router::new()
        .route("/write", post(write_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
