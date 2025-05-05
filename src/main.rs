use axum::{Json, Router};
use axum::routing::get;
use axum::http::StatusCode;
use serde::Serialize;

// ------
//  Main
// ------
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root));
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ------------------
//  REQUEST HANDLERS
// ------------------
#[derive(Serialize)]
struct Response {
    message: &'static str,
}

async fn root() -> (StatusCode, Json<Response>) {
    let response = Response {
        message: "This app tracks fascists!",
    };
    
    (StatusCode::OK, Json(response))
}
// async fn root() -> &'static str {
//     "This app tracks fascists!"
// }
