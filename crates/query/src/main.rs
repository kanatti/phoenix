use arrow::array::RecordBatch;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

mod engine;

use engine::{QueryEngine, QueryError};

#[derive(Clone)]
struct AppState {
    query_engine: Arc<QueryEngine>,
}

#[derive(Deserialize)]
struct QueryRequest {
    query: String,
}

#[derive(Serialize)]
enum QueryResponse {
    Success(SuccessResponse),
    Error(ErrorResponse),
}

#[derive(Serialize)]
struct SuccessResponse {
    results: Vec<serde_json::Value>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct StatusResponse {
    status: String,
}

async fn execute_query(
    State(state): State<AppState>,
    Json(payload): Json<QueryRequest>,
) -> (StatusCode, Json<QueryResponse>) {
    let result = state.query_engine.execute_query(&payload.query).await;

    match result {
        Err(e) => {
            let status = match e {
                QueryError::ParseError(_) => StatusCode::BAD_REQUEST,
                QueryError::ExecutionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                QueryError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
    
            let response = QueryResponse::Error(ErrorResponse {
                error: e.to_string(),
            });
            return (status, Json(response));
        },
        Ok(r) => {
            let results = arrow_to_json(&r);

            match results {
                Err(e) => {
                    let status = StatusCode::INTERNAL_SERVER_ERROR;

                    let response = QueryResponse::Error(ErrorResponse {
                        error: e.to_string(),
                    });

                    return (status, Json(response));
                },
                Ok(results) => {
                    let response = QueryResponse::Success(SuccessResponse { results });

                    return (StatusCode::OK, Json(response))
                }
            }
        }
    }
}

fn arrow_to_json(
    batches: &[RecordBatch],
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let mut writer = arrow_json::ArrayWriter::new(Vec::new());
    for batch in batches {
        writer.write(batch)?;
    }
    writer.finish()?;

    let json_values: Vec<serde_json::Value> = serde_json::from_slice(&writer.into_inner())?;

    Ok(json_values)
}

async fn get_status() -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "OK".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let query_engine = Arc::new(QueryEngine::new());

    let app_state = AppState { query_engine };

    let app = Router::new()
        .route("/query", post(execute_query))
        .route("/status", get(get_status))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
