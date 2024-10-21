mod routes;

use crate::routes::screenshot_route::screenshot_handler;
use axum::{
    routing::get, Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/screenshot", get(screenshot_handler));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on: 3000");

    axum::serve(listener, app).await.unwrap();
}


