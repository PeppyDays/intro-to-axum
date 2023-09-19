use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MirrorRequest {
    message: String,
}

#[derive(Serialize)]
pub struct MirrorResponse {
    message: String,
    user_id: u32,
}

pub async fn mirror(Json(body): Json<MirrorRequest>) -> Json<MirrorResponse> {
    Json(MirrorResponse {
        message: body.message,
        user_id: 32,
    })
}
