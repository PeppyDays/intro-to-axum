use axum::http::HeaderMap;

pub async fn auth(headers: HeaderMap) -> String {
    headers.get("auth").unwrap().to_str().unwrap().to_string()
}
