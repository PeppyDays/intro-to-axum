mod routes;

use axum::Server;
use routes::create_routes;

pub async fn run() {
    let app = create_routes();

    Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
