use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

mod store;
mod util;

use store::{LocalStore, RemoteStore, Store};

#[derive(Deserialize)]
struct WriteRequest {
    data: Vec<Value>,
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
) -> (StatusCode, Json<WriteResponse>) {
    let data = util::read_record_batch_from_json(&req.data);

    if let Err(e) = data {
        let response = Json(WriteResponse {
            status: format!("error: {}", e),
        });
        return (StatusCode::BAD_REQUEST, response);
    }

    let store = state.store;
    if let Err(e) = store.write(data.unwrap(), &req.path).await {
        let response = Json(WriteResponse {
            status: format!("error: {}", e),
        });
        return (StatusCode::INTERNAL_SERVER_ERROR, response);
    }

    if let Err(e) = store.notify_catalog(&req.path).await {
        let response = Json(WriteResponse {
            status: format!("error: {}", e),
        });
        return (StatusCode::INTERNAL_SERVER_ERROR, response);
    }

    (
        StatusCode::OK,
        Json(WriteResponse {
            status: "ok".to_string(),
        }),
    )
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
