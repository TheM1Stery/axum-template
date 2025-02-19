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
        </head>
        <body>
            {body}
        </body>
        </html>
    }
}
