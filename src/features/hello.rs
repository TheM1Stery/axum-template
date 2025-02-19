use axum::response::IntoResponse;
use hypertext::{html_elements, rsx, Renderable};

pub async fn hello() -> impl IntoResponse {
    rsx! {
        <div>
            Hello!
        </div>
    }
    .render()
}
