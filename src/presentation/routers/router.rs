use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde_derive::{Deserialize, Serialize};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    tracing::debug!("root recive get request ");
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    tracing::debug!("create_user recive post request ");
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}