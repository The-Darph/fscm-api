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

pub async fn all(/*State(state): State<Arc<ApplicationState>>*/) -> Json<Value> {
    Json(json!({ "message": "Unimplemented. All events will be returned here." }))
}
