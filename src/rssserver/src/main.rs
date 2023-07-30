mod api;
mod models;
mod response;

use axum::{routing::{get, post}, Router, ServiceExt};
use reqwest::Client;
use std::net::SocketAddr;
use tower::layer::Layer;
use tower_http::{normalize_path::NormalizePathLayer, trace::TraceLayer, cors::CorsLayer};

use crate::models::AppState;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "0");

    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    let app_state = AppState { client: Client::new() };

    let app = NormalizePathLayer::trim_trailing_slash().layer(
        Router::new()
            .route("/test", get(api::root::get))
            .route("/*path", get(api::static_content::static_path))
            .route("/", get(api::static_content::static_root))
            .route("/error", get(api::root::error))
            .route("/rss", get(api::rss::view))
            .route("/rss/blogs/:id", get(api::rss::view_blog))
            .route("/rss/blogs/:id/articles/*path", get(api::rss::view_article))
            .route("/blog", post(api::blog::view_blog))
            .with_state(app_state)
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http()),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3031));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}