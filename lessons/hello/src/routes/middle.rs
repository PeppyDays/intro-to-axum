use std::sync::Arc;

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};

use super::SharedData;

pub async fn middle<'a>(Extension(shared_data): Extension<Arc<SharedData<'a>>>) -> &'a str {
    shared_data.message
}

pub async fn middle_2<'a>(State(shared_data): State<Arc<SharedData<'a>>>) -> &'a str {
    shared_data.message
}

#[derive(Clone)]
pub struct HeaderMessage(String);

pub async fn header(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}

pub async fn extract_message<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let message = request
        .headers()
        .get("message")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_e| StatusCode::BAD_REQUEST)?
        .to_owned();

    let extensions = request.extensions_mut();
    extensions.insert(HeaderMessage(message));

    Ok(next.run(request).await)
}
