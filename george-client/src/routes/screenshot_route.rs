use axum::body::Bytes;
use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use screenshots::image::{ImageOutputFormat, RgbaImage};
use screenshots::Screen;
use std::io::Cursor;


pub async fn screenshot_handler() -> impl IntoResponse {
    let screens = match Screen::all() {
        Ok(s) => s,
        Err(e) => {
            let string = format!("Failed to get screens: {}", e);
            println!("{}", string);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, string));
        }
    };
    let screen = &screens[0];
    let image: RgbaImage = match screen.capture() {
        Ok(img) => img,
        Err(e) => {
            let string1 = format!("Failed to capture image: {}", e);
            println!("{}", string1);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, string1));
        }
    };

    let mut buffer = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut buffer), ImageOutputFormat::Png)
        .expect("Failed to encode image to PNG");

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("image/png"),
    );

    Ok((headers, Bytes::from(buffer)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::response::Response;
    use axum::body::to_bytes;

    #[tokio::test]
    async fn test_screenshot_handler_success() {
        let response: Response = screenshot_handler().await.into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let headers = response.headers();
        assert_eq!(
            headers.get(axum::http::header::CONTENT_TYPE),
            Some(&axum::http::HeaderValue::from_static("image/png"))
        );

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert!(!body.is_empty());


        let png_signature = &[137, 80, 78, 71, 13, 10, 26, 10];
        assert_eq!(&body[0..8], png_signature);
    }
}