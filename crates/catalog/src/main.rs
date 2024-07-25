use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
struct Field {
    field: String,
    #[serde(rename = "type")]
    field_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Schema {
    fields: Vec<Field>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TableRequest {
    name: String,
    schema: Schema,
}

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

async fn create_metadata_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS table_metadata (
            name TEXT PRIMARY KEY,
            schema TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn store_table_metadata(
    State(state): State<AppState>,
    Json(payload): Json<TableRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let schema_json = serde_json::to_string(&payload.schema)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let result = sqlx::query(
        "INSERT INTO table_metadata (name, schema) VALUES (?, ?)
         ON CONFLICT(name) DO UPDATE SET schema = excluded.schema",
    )
    .bind(&payload.name)
    .bind(&schema_json)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::CREATED)
    } else {
        Ok(StatusCode::OK)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the SQLite database
    let db_options =
        SqliteConnectOptions::from_str("sqlite://data/table_metadata.db")?.create_if_missing(true);
    let pool = SqlitePool::connect_with(db_options).await?;

    // Create the metadata table if it doesn't exist
    create_metadata_table(&pool).await?;

    // Create the Axum app
    let app_state = AppState { pool };
    let app = Router::new()
        .route("/tables", post(store_table_metadata))
        .with_state(app_state);

    // Run the server
    println!("Server running on http://localhost:3002");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
