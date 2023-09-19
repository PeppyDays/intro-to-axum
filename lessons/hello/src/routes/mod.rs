mod agent;
mod auth;
mod hello;
mod json;
mod middle;
mod mirror;
mod param;
mod status;
mod thing;
mod validation;

use std::sync::Arc;

use crate::routes::agent::agent;
use crate::routes::auth::auth;
use crate::routes::hello::hello;
use crate::routes::json::get_json;
use crate::routes::middle::{extract_message, header, middle, middle_2};
use crate::routes::mirror::mirror;
use crate::routes::param::param;
use crate::routes::status::{always_errors, return_201};
use crate::routes::thing::thing;
use crate::routes::validation::validate;
use axum::middleware;
use axum::{
    http::Method,
    routing::{get, post},
    Extension, Router,
};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData<'a> {
    pub message: &'a str,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = Arc::new(SharedData {
        message: "hello from shared!",
    });

    let new_shared_data = Arc::new(SharedData {
        message: "hi there?",
    });

    Router::new()
        .route("/header", get(header))
        .route_layer(middleware::from_fn(extract_message))
        .route("/", get(hello))
        .route("/mirror", post(mirror))
        .route("/thing/:id", get(thing))
        .route("/param", get(param))
        .route("/agent", get(agent))
        .route("/auth", get(auth))
        .route("/middle", get(middle))
        .route("/middle_2", get(middle_2))
        .route("/always_errors", get(always_errors))
        .route("/return_201", post(return_201))
        .route("/get_json", get(get_json))
        .route("/validate", post(validate))
        .layer(cors)
        .layer(Extension(shared_data))
        .with_state(new_shared_data)
}
