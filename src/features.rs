use axum::{routing::get, Router};
use hello::hello;
use hypertext::{html_elements, rsx, Renderable};
mod hello;

pub fn routes() -> Router {
    Router::new().route("/", get(hello))
}

fn document(body: impl Renderable) -> impl Renderable {
    rsx! {
        <!doctype html>
        <html>
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>Chat Rust</title>
            //temporary(will use bun)
            <script src="https://unpkg.com/htmx.org@2.0.4"></script>
            <script src="https://unpkg.com/htmx-ext-ws@2.0.1/ws.js"></script>
        </head>
        <body>
            {body}
        </body>
        </html>
    }
}
