use axum::Router;
mod handlers;
mod v1;
use crate::state::ApplicationState;
use std::sync::Arc;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new().nest("/v1", v1::configure(state))
}