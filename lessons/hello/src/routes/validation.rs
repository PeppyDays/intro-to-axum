use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseUser {
    username: String,
    password: Option<String>,
}

pub async fn validate(Json(data): Json<RequestUser>) -> Json<ResponseUser> {
    Json(ResponseUser {
        username: data.username,
        password: data.password,
    })
}
