use axum::{extract::State, Json};
use std::sync::Arc;
use serde_json::{json, Value};
use crate::state::ApplicationState;
use crate::db::subtypes::get_all_subtypes;

pub async fn all(
    State(state): State<Arc<ApplicationState>>,
) -> Json<serde_json::Value> {
    let mut conn = state.db_pool.get().expect("DB connection failed");

    match get_all_subtypes(&mut conn) {
        Ok(subtypes) => Json(serde_json::json!({ "subtypes": subtypes })),
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            Json(serde_json::json!({ "error": "Failed to fetch subtypes" }))
        }
    }
}
