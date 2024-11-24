use axum::response::Html;

pub async fn root_handler() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>George Client Screenshot</title>
            <style>
                body {
                    background-color: #f5f5f5;
                }
            </style>
            <script>
                function refreshImage() {
                    const img = document.getElementById('screenshot');
                    img.src = '/screenshot?' + new Date().getTime();
                }

                setInterval(refreshImage, 250);
            </script>
        </head>
        <body>
            <img id="screenshot" src="/screenshot" alt="Screenshot" width="1024" height="768">
        </body>
        </html>
        "#
    )
}