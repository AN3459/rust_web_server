use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{Response, StatusCode},
    Json,
};
use serde_derive::{Deserialize, Serialize};

use crate::AppState;

// #[derive(Debug, Deserialize)]
// pub struct LoginPayload {
//     code: String,
// }

// pub struct UserController {}

// impl UserController {
//     pub async fn login() -> Response<Json<()>> {
//         todo!()
//     }
// }
use crate::infrastructure::repository::user_repository;

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path((id,)): Path<(String,)>,
) -> (StatusCode, Json<User>) {
    tracing::debug!("handler function:{},parameters:{}", "get_user", id);
    let user_info = user_repository::find_user_by_id(state, id.clone()).await.unwrap();
    let user =
        User { id, username: String::from("user1"), config_info: user_info.unwrap().open_id };
    tracing::debug!("handler function:{},handle result:{:?}", "get_user", user);
    (StatusCode::OK, Json(user))
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    tracing::debug!("create_user recive post request ");
    // insert your application logic here
    let user = User {
        id: String::from("1337"),
        username: payload.open_id.clone(),
        config_info: "ABC".to_string(),
    };
    let _ = user_repository::save(state, payload.open_id.clone()).await;
    // = user_repository.save(payload.open_id.clone()).await;
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    open_id: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    id: String,
    username: String,
    config_info: String,
}
