use axum::{
    Json
};

pub async fn handler() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, World!"),
    })
}

#[derive(serde::Serialize)]
pub struct Message {
    message: String,
}
