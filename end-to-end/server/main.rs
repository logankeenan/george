use axum::{
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/submit", post(submit));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("running end-to-end server at http://localhost:3001");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> {
    Html(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Name Form</title>
        </head>
        <body>
            <form action="/submit" method="post">
                <label for="name">Name:
                    <input type="text" id="name" name="name">
                </label>
                <br>
                <button type="submit">Submit</button>
            </form>
        </body>
        </html>
        "#
            .to_string(),
    )
}

#[derive(Deserialize)]
struct WebForm {
    name: String,
}

async fn submit(Form(form): Form<WebForm>) -> Html<String> {
    if form.name == "Ada Lovelace" {
        Html("<h1>Success</h1>".to_string())
    } else {
        Html("<h1>Failure</h1>".to_string())
    }
}
