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
            <link href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css" rel="stylesheet">
            <title>Form Submission</title>
        </head>
        <body>
            <div class="container mt-5" style="max-width: 500px">
                <form autocomplete="off" action="/submit" method="post" class="p-4">
                    <div class="form-group">
                        <label for="name">Name</label>
                        <input type="text" class="form-control" id="name" name="name">
                    </div>

                    <div class="form-group">
                        <label for="phone">Phone</label>
                        <input type="tel" class="form-control" id="phone" name="phone">
                    </div>

                    <div class="form-group">
                        <label for="email">Email</label>
                        <input type="email" class="form-control" id="email" name="email">
                    </div>

                    <div class="form-check mb-3">
                        <input type="checkbox" class="form-check-input" id="first_programmer" name="first_programmer" value="true">
                        <label class="form-check-label" for="first_programmer">First Programmer</label>
                    </div>

                    <div class="form-group">
                        <div class="form-check">
                            <input type="radio" class="form-check-input" id="analytical_engine" name="work" value="analytical_engine">
                            <label class="form-check-label" for="analytical_engine">Analytical Engine</label>
                        </div>
                        <div class="form-check">
                            <input type="radio" class="form-check-input" id="programming" name="work" value="programming">
                            <label class="form-check-label" for="programming">Programming</label>
                        </div>
                    </div>

                    <button type="submit" class="btn btn-primary">Submit</button>
                </form>
            </div>
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