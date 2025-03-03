use axum::response::IntoResponse;
use hypertext::{Renderable, html_elements, rsx};

use super::document;

pub async fn hello() -> impl IntoResponse {
    let body = rsx! {
        <div>
            Axum Template
        </div>
    };

    document(body, "Axum Template").render()
}
