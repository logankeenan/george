use axum::Json;
use axum::response::IntoResponse;
use enigo::{Enigo, Keyboard, Settings};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct TypePayload {
    text: String,
}

pub async fn type_handler(Json(payload): Json<TypePayload>) -> impl IntoResponse {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // Type out the provided text
    enigo.text(&payload.text).unwrap();

    Json(json!({
        "status": "typed",
        "text": payload.text
    }))
}