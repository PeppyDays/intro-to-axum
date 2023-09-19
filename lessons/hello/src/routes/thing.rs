use axum::extract::Path;

pub async fn thing(Path(id): Path<i32>) -> String {
    format!("hello, {}", id)
}
