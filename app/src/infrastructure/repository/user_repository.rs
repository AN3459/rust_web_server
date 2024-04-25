use crate::common::database::db_entity::prelude::Users;
use crate::common::database::db_entity::users;
use crate::AppState;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DbErr, EntityTrait,
};
use std::sync::Arc;

// use serde_derive::{Deserialize, Serialize};
// use gremlin_client::{aio::GremlinClient, Vertex};
// use tokio_stream::StreamExt;

// pub struct UserRepository {
//     state: AppState,
// }

// #[derive(Clone)]
// struct AppState {
//     conn: DatabaseConnection,
// }

// impl UserRepository {
//     pub async fn new() -> UserRepository {
//         let connection_result = DbConnection::new().await.expect("Database connection failed");

//         UserRepository { state: AppState { conn: connection_result } }
//     }

//     // pub async fn read_one() -> User{
//     //     User {id:"123".to_string(),username:"123".to_string(),config_info:"123".to_string()}
//     // }

//     pub async fn find_user_by_id(&self, id: String) -> Result<Option<users::Model>, DbErr> {
//         Users::find_by_id(id).one(&self.state.conn).await
//     }

//     // pub async fn gremlin_test(&self)-> Result<Vec<Vertex>, Box<dyn std::error::Error>> {
//     //     let client = GremlinClient::connect("wss://db-neptune-1.cluster-ro-c8q0jluobu0c.ap-northeast-1.neptune.amazonaws.com:8182/gremlin").await?;
//     //     let results = client.execute("g.V(param)", &[("param", &1)]).await?
//     //         .filter_map(Result::ok)
//     //         .map(|f| f.take::<Vertex>())
//     //         .collect::<Result<Vec<Vertex>, _>>().await?;
//     //     println!("{:?}", results);
//     //     Ok(results)
//     // }
//     pub async fn save(&self, open_id: String) -> Result<users::ActiveModel, DbErr> {
//         users::ActiveModel { id: NotSet, open_id: Set(open_id.to_owned()), ..Default::default() }
//             .save(&self.state.conn)
//             .await
//     }
// }

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

pub async fn find_user_by_id(
    state: Arc<AppState>,
    id: String,
) -> Result<Option<users::Model>, DbErr> {
    Users::find_by_id(id).one(&state.db).await
}

pub async fn save(state: Arc<AppState>, open_id: String) -> Result<users::ActiveModel, DbErr> {
    let res =
        users::ActiveModel { id: NotSet, open_id: Set(open_id.to_owned()), ..Default::default() }
            .save(&state.db)
            .await;
    tracing::debug!("Save OK ");
    res
}
