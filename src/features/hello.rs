use axum::response::IntoResponse;
use hypertext::{html_elements, rsx, Renderable};

use super::document;

pub async fn hello() -> impl IntoResponse {
    let body = rsx! {
        <div>
            Hello!
        </div>
    };

    document(body).render()
}
