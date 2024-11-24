use axum::http::StatusCode;

pub async fn healthz() -> StatusCode {
    StatusCode::OK
}