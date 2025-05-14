use super::handlers;
use axum::routing::get;
use axum::Router;
use crate::state::ApplicationState;
use std::sync::Arc;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route(
            "/hello", 
            get(handlers::hello::hello).with_state(state.clone()),
        )
        .route(
            "/events",
            get(handlers::events::all).with_state(state.clone()),
        )
}
