use axum::{routing::get, Router};
use hello::hello;
mod hello;

pub fn routes() -> Router {
    Router::new().route("/", get(hello))
}
