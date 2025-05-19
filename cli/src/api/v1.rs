use super::handlers;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use crate::state::ApplicationState;
use std::sync::Arc;
// use crate::db::{establish_pool, DbPool};

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
        .route(
            "/events",
            post(handlers::events::insert).with_state(state.clone()),
        )
        .route(
            "/events/{id}",
            get(handlers::events::one).with_state(state.clone()),
        )
}
