use axum::{
    response::IntoResponse,
    extract::{State, Query, Multipart, Path},
    // Router,
    Json,
    http::StatusCode,
};
use std::sync::Arc;
use serde_json::{Value, json};
use crate::state::ApplicationState;
use crate::db::events::{get_all_events, insert_event, get_event_by_id};
use crate::model::{EventWithRelations, Event};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub limit: Option<String>,
    pub page: Option<String>,
}

#[derive(Deserialize)]
pub struct UploadAuth {
    token: String,
}

pub async fn all(
    State(state): State<Arc<ApplicationState>>,
    Query(params): Query<PaginationParams>,
) -> Json<Vec<EventWithRelations>> {
    let mut conn = state.db_pool.get().expect("DB connection failed");
    let events = get_all_events(&mut conn, params.limit, params.page).expect("Query failed");
    Json(events)
}

pub async fn insert(
    State(state): State<Arc<ApplicationState>>,
    Query(auth): Query<UploadAuth>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    // Check for token first
    if auth.token != state.upload_secret {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "unauthorized" })),
        );
    }

    let mut conn = match state.db_pool.get() {
        Ok(c) => c,
        Err(_) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "status": "error", "message": "Database unavailable" }))),
    };

    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().map(str::to_owned).unwrap_or_else(|| "upload.csv".into());
        let data = field.bytes().await.unwrap();

        match insert_event(&mut conn, &data) {
            Ok(count) => {
                return (StatusCode::OK, 
                        Json(json!({
                            "status": "ok",
                            "filename": filename,
                            "inserted": count
                        })))
            }
            Err(err) => {
                eprintln!("CSV insert error: {:?}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "status": "error",
                            "filename": filename,
                            "message": err.to_string()
                        })))
            }
        }
    }

    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "status": "no file uploaded" })))
}

pub async fn one(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let mut conn = match state.db_pool.get() {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "db error" }))).into_response(),
    };

    match get_event_by_id(&mut conn, id) {
        Ok(Some(event)) => Json(event).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({ "error": "event not found" }))).into_response(),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "unexpected failure" }))).into_response()
        }
    }
}
