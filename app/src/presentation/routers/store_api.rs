use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde_derive::{Deserialize, Serialize};

use crate::AppState;

pub fn create_store_api_router(app_state: Arc<AppState>) -> Router {
    Router::new().route("/test", get(handler)).with_state(app_state)
}

async fn handler() -> (StatusCode, String) {
    (StatusCode::CREATED, String::from("hello"))
}
