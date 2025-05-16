use axum::{
    // body::Body,
    // routing::get,
    response::Json,
    // extract::State,
    // Router,
};
// use crate::state::ApplicationState;
// use axum::extract::State;
// use std::sync::Arc;
use serde_json::{Value, json};
use crate::establish_connection;

pub async fn all(/*State(state): State<Arc<ApplicationState>>*/) -> Json<Value> {
    Json(json!({ "message": "all() Unimplemented. All events will be returned here." }))
}

pub async fn insert() -> Json<Value> {
    let connection = &mut establish_connection();
    Json(json!({ "message": "insert() Unimplemented. Events will be inserted here." }))
}

pub async fn one() -> Json<Value> {
    Json(json!({ "message": "one() Unimplemented. One event by id will be returned here." }))
}
