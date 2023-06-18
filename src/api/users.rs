use axum::{
    extract::{Path, Query},
    Json, Extension
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

use crate::models::auth_context::AuthContext;


pub async fn get_user(
    Extension(auth_context): Extension<AuthContext>,
    Path(id): Path<String>,
    pagination: Query<Pagination>,
) -> Json<Message> {
    info!(id, pagination.page, pagination.per_page);
    
    Json(Message {
        message: id,
        auth_context: auth_context,
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
    message: String,
    auth_context: AuthContext
}

#[derive(serde::Serialize)]
pub struct PostMessage {
    message: String,
    email: String,
    password: String
}
