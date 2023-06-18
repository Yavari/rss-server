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

#[derive(serde::Serialize)]
pub struct Message {
    message: String,
}
