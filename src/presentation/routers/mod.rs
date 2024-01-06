use axum::Router;

pub mod user_api;

pub fn create_router() -> Router {
    let user_api = user_api::create_user_api_router();

    Router::new()
    .nest("/users", user_api)
}