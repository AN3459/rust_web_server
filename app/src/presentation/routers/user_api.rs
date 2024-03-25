use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde_derive::{Deserialize, Serialize};

use crate::common::config::config::RUNNING_ENV;
use crate::infrastructure::repository::user_repository::UserRepository;

pub fn create_user_api_router() -> Router {
    Router::new().route("/:id", get(get_user)).route("/users", post(create_user))
}

async fn get_user(Path((id,)): Path<(String,)>) -> (StatusCode, Json<User>) {
    tracing::debug!("handler function:{},parameters:{}", "get_user", id);
    let user_repository = UserRepository::new().await;
    let user_info = user_repository.find_user_by_id(String::from("123")).await.unwrap();
    let user =
        User { id, username: String::from("user1"), config_info: user_info.unwrap().open_id };
    tracing::debug!("handler function:{},handle result:{:?}", "get_user", user);
    (StatusCode::OK, Json(user))
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    tracing::debug!("create_user recive post request ");
    // insert your application logic here
    let user = User {
        id: String::from("1337"),
        username: payload.username,
        config_info: RUNNING_ENV.to_string(),
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Debug, Serialize)]
struct User {
    id: String,
    username: String,
    config_info: String,
}
