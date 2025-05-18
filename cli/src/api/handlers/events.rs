use axum::{
    // body::Body,
    // routing::get,
    // response::Json,
    extract::State,
    extract::Query,
    // Router,
    Json,
};
// use crate::state::ApplicationState;
use std::sync::Arc;
use serde_json::{Value, json};
use crate::db::events::*;

use crate::state::ApplicationState;
use crate::db::events::get_all_events;
use crate::model::Event;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub limit: Option<String>,
    pub page: Option<String>,
}

pub async fn all(
    State(state): State<Arc<ApplicationState>>,
    Query(params): Query<PaginationParams>,
) -> Json<Vec<Event>> {
    let mut conn = state.db_pool.get().expect("DB connection failed");
    let events = get_all_events(&mut conn, params.limit, params.page).expect("Query failed");
    Json(events)
}

pub async fn insert() -> Json<Value> {
    // let connection = &mut establish_connection();
    Json(json!({ "message": "insert() Unimplemented. Events will be inserted here." }))
}

pub async fn one() -> Json<Value> {
    Json(json!({ "message": "one() Unimplemented. One event by id will be returned here." }))
}
