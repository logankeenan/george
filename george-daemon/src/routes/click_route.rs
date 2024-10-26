use axum::Json;
use axum::response::IntoResponse;
use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct ClickPayload {
    x: i32,
    y: i32,
}


pub async fn click_handler(Json(payload): Json<ClickPayload>) -> impl IntoResponse {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    enigo.move_mouse(payload.x, payload.y, Coordinate::Abs).unwrap();
    enigo.button(Button::Left, Direction::Click).unwrap();

    Json(json!({
        "status": "clicked",
        "x": payload.x,
        "y": payload.y
    }))
}
