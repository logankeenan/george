mod routes;

use crate::routes::click_route::click_handler;
use crate::routes::screenshot_route::screenshot_handler;
use axum::routing::post;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/screenshot", get(screenshot_handler))
        .route("/click", post(click_handler));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on: 3000");

    axum::serve(listener, app).await.unwrap();
}


