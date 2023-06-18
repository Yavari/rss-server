mod api;
mod middlewares;
mod models;

use axum::{
    routing::{get, post},
    Router, 
    ServiceExt, middleware,
};
use std::{net::SocketAddr};

use tower_http::{trace::TraceLayer, normalize_path::NormalizePathLayer};
use tower::layer::Layer;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "0");

    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();

    let app = NormalizePathLayer::trim_trailing_slash().layer(
        Router::new()
        .route("/users/:id", get(api::users::get_user))
        .route("/users", post(api::users::create_user))
        .route("/", get(api::root::get))
        .route("/error", get(api::root::error))
        .route_layer(middleware::from_fn(middlewares::azure_auth_middleware::azure_auth_middleware))
        .layer(TraceLayer::new_for_http())
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
