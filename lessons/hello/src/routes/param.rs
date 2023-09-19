use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ParamRequest {
    id: u32,
}

#[derive(Serialize)]
pub struct ParamResponse {
    id: u32,
    name: String,
}

pub async fn param(Query(param): Query<ParamRequest>) -> Json<ParamResponse> {
    Json(ParamResponse {
        id: param.id,
        name: String::from("Arine"),
    })
}
