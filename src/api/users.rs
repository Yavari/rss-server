use axum::{
    extract::{Path, Query},
    Json
};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<usize>,
    per_page: Option<usize>,
}

#[derive(Deserialize)]
pub struct CreateUser {
    email: String,
    password: String,
}

use tracing::{info};

pub async fn get_user(
    Path(id): Path<String>,
    pagination: Query<Pagination>
) -> Json<Message> {
    info!(id, pagination.page, pagination.per_page);
    Json(Message {
        message: id,
    })
}

pub async fn create_user(
    Json(payload): Json<CreateUser>
) -> Json<PostMessage> {
    let id = "1".to_string();
    Json(PostMessage {
        message: id,
        email: payload.email,
        password: payload.password
    })
}

#[derive(serde::Serialize)]
pub struct Message {
    message: String
}

#[derive(serde::Serialize)]
pub struct PostMessage {
    message: String,
    email: String,
    password: String
}
