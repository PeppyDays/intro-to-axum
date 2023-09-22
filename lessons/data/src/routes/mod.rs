mod custom_json_extractor;
mod hello_world;
mod task;
mod validate_with_serde;

use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};

use custom_json_extractor::custom_json_extractor;
use hello_world::hello_world;
use sea_orm::DatabaseConnection;
use task::*;
use validate_with_serde::validate_with_serde;

pub async fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/hello_world", get(hello_world))
        .route("/validate_with_serde", post(validate_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
        // .route("/tasks", post(create_task).get(get_all_tasks))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:id", get(get_task))
        .route("/tasks/:id", put(atomic_update))
        .route("/tasks/:id", patch(partial_update))
        .route("/tasks/:id", delete(delete_task))
        .with_state(database)
}
