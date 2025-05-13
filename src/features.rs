use axum::{Router, routing::get};
use hello::hello;
use hypertext::{Renderable, html_elements, rsx};
mod hello;

pub fn routes() -> Router {
    Router::new().route("/", get(hello))
}

fn document(body: impl Renderable, title: &str) -> impl Renderable {
    rsx! {
        <!doctype html>
        <html>
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>{title}</title>
        </head>
        <body>
            {body}
        </body>
        </html>
    }
}
