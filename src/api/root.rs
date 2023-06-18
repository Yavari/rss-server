use axum::{
    Json
};

pub async fn get() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, World!"),
    })
}


pub async fn error() -> Json<Message> {
    panic!("This is a test")
}


#[derive(serde::Serialize)]
pub struct Message {
    message: String,
}
