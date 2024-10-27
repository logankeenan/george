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
        </head>
        <body>
            <form action="/submit" method="post">
                <label for="name">Name:
                    <input type="text" id="name" name="name">
                </label>
                <br>
                <br>
                <label for="phone">Phone:
                    <input type="tel" id="phone" name="phone">
                </label>
                <br>
                <br>
                <label for="email">Email:
                    <input type="email" id="email" name="email">
                </label>
                <br>
                <br>
                <label for="first_programmer">First Programmer:
                    <input type="checkbox" id="first_programmer" name="first_programmer" value="true">
                </label>
                <br>
                <br>
                <fieldset>
                    <legend>Work:</legend>
                    <input type="radio" id="analytical_engine" name="work" value="analytical_engine">
                    <label for="analytical_engine">Analytical Engine</label>
                    <input type="radio" id="programming" name="work" value="programming">
                    <label for="programming">Programming</label>
                </fieldset>
                <br>
                <button type="submit">Submit</button>
            </form>
        </body>
        </html>
        "#
            .to_string(),
    )
}

#[derive(Deserialize, Debug)]
struct WebForm {
    name: String,
    phone: String,
    email: String,
    #[serde(default)]
    first_programmer: bool,
    work: Option<String>,
}

async fn submit(Form(form): Form<WebForm>) -> Html<String> {
    println!("form {:?}", form);

    if form.name == "Ada Lovelace" &&
        form.phone == "5554443333" &&
        form.email == "ada@email.com" &&
        form.first_programmer &&
        form.work.unwrap_or(String::from("")) == "programming"
    {
        Html("<h1>Success</h1>".to_string())
    } else {
        Html("<h1>Failure</h1>".to_string())
    }
}