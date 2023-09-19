use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        message: "hello?".to_string(),
        count: 48723,
        username: "arine".to_string(),
    };

    Json(data)
}
