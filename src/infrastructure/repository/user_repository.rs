use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
// use serde_derive::{Deserialize, Serialize};
use crate::common::database::connection::DbConnection;
use crate::common::database::db_entity::{post, post::Entity as Post};
pub struct UserRepository {
    prefix: String,
    state: AppState,
}

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

impl UserRepository {
    pub async fn new() -> UserRepository {
        let connection_result = DbConnection::new().await.expect("Database connection failed");

        UserRepository { prefix: String::from("REPO"), state: AppState { conn: connection_result } }
    }

    // pub async fn read_one() -> User{
    //     User {id:"123".to_string(),username:"123".to_string(),config_info:"123".to_string()}
    // }

    pub async fn find_post_by_id(&self, id: i32) -> Result<Option<post::Model>, DbErr> {
        Post::find_by_id(id).one(&self.state.conn).await
    }
}

// #[derive(Debug, Deserialize)]
// struct CreateUser {
//     username: String,
// }

// #[derive(Debug, Serialize)]
// struct User {
//     id: String,
//     username: String,
//     config_info: String,
// }

// async fn get_user(Path((id,)): Path<(String,)>) -> (StatusCode, Json<User>) {
//     tracing::debug!("handler function:{},parameters:{}", "get_user", id);

//     let user = User { id, username: String::from("user1"), config_info: RUNNING_ENV.to_string() };

//     tracing::debug!("handler function:{},handle result:{:?}", "get_user", user);
//     (StatusCode::OK, Json(user))
// }
