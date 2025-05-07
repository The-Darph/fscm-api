use anyhow::Context;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use axum::extract::Query;

use std::net::SocketAddr;
use std::collections::HashMap;
use serde::Serialize;


#[derive(Serialize)]
struct Response {
    message: &'static str,
    results: i32,
}

struct AppError(anyhow::Error);

// This allows ? to automatically convert anyhow::Error to AppError
impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", self.0)).into_response()
    }
}

// ------------------
//  REQUEST HANDLERS
// ------------------
async fn root(Query(params): Query<HashMap<String, String>>) -> Result<(StatusCode, Json<Response>), AppError> {
    let raw_value = params.get("results").map(|s| s.as_str());
    // let parsed_value = get_events(raw_value);
    let response = Response {
        message: "This app tracks fascists",
        results: get_events(raw_value),//.context("Failed to grab records for a response")?,
    };           // CHANGE fn NAME TO SOMETHING THAT MAKES SENSE
    
    Ok((StatusCode::OK, Json(response)))
}

// ----------------------------------------
//           Support Functions
// ----------------------------------------
// These break out logic so not everything
// is crammed into the request handlers.
// Mostly models and controllers until we 
// break this into more files.
fn get_events(input: Option<&str>) -> i32 {
    input
        .and_then(|s| s.trim().parse::<i32>().ok())
        .unwrap_or(1000)
} // CHANGEME: Update to just sanitize the value then get data in the handler or another function

// ------
//  Main
// ------
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(root));
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("Failed to bind to TCP listener")?;

    axum::serve(listener, app)
        .await
        .context("axum::serve failed")?;

    Ok(())
}
