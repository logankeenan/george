mod routes;

use crate::{
    routes::click_route::click_handler,
    routes::screenshot_route::screenshot_handler,
    routes::type_route::type_handler,
    routes::root_route::root_handler,
};
use axum::routing::post;
use axum::{routing::get, Router};
use crate::routes::health_route::healthz;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/screenshot", get(screenshot_handler))
        .route("/click", post(click_handler))
        .route("/type", post(type_handler))
        .route("/healthz", get(healthz));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on: 3000");

    axum::serve(listener, app).await.unwrap();
}



