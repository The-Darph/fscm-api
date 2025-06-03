use super::handlers;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use crate::state::ApplicationState;
use std::sync::Arc;
// use crate::db::{establish_pool, DbPool};
// use crate::api::handlers::{types_handler, subtypes_handler};

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
        .route(
            "/types",
            get(handlers::types::all).with_state(state.clone()),
        )
        .route(
            "/subtypes",
            get(handlers::subtypes::all).with_state(state.clone()),
        )
}
