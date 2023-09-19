use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn always_errors() -> Result<Response, StatusCode> {
    Err(StatusCode::IM_A_TEAPOT)
}

pub async fn return_201() -> Response {
    (StatusCode::CREATED, "This is a 201".to_string()).into_response()
}
