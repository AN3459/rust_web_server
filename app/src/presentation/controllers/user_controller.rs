use axum::{http::Response, Json};
use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    code: String
}

pub struct UserController {

}

impl UserController {
    pub async fn login()-> Response<Json<()>>{
        todo!()
    }
}


