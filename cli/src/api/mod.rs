use axum::{Router, routing::get};
mod handlers;
mod v1;
use crate::state::ApplicationState;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;

// #[tokio::main]
pub fn configure(state: Arc<ApplicationState>) -> Router {
    // Setup CORS middleware
    let allowed_production = "https://your-production-site.com".parse().expect("Invalid origin URL");
    let allowed_development = "http://localhost:9001".parse().expect("Invalid origin URL");
    let cors = CorsLayer::new()
        .allow_origin([allowed_production, allowed_development])
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new().nest("/v1", v1::configure(state)).layer(cors)
}
