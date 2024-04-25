use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub mod store_api;
pub mod user_api;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let user_api = user_api::create_user_api_router(app_state.clone());
    let store_api = store_api::create_store_api_router(app_state.clone());

    Router::new().nest("/users", user_api).nest("/store", store_api)
}
