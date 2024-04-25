use crate::{
    presentation::controllers::user_controller::{create_user, get_user},
    AppState,
};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn create_user_api_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/:id", get(get_user))
        .route("/users", post(create_user))
        .with_state(app_state)
}
