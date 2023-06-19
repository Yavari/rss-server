mod api;
mod middlewares;
mod models;

use axum::{
    middleware,
    routing::{get, post},
    Router, ServiceExt,
};
use azure_jwt::AzureAuth;
use std::{net::SocketAddr, sync::{Arc, Mutex}};
use tokio::task::spawn_blocking;

use tower::layer::Layer;
use tower_http::{normalize_path::NormalizePathLayer, trace::TraceLayer};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "0");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let azure_auth = spawn_blocking(|| get_az_auth())
        .await
        .expect("AzureAuth::new task spawn failed");

    let state = models::app_state::AppState { 
        azure_auth: Arc::new(Mutex::new(azure_auth)) 
    };

    let app = NormalizePathLayer::trim_trailing_slash().layer(
        Router::new()
            .route("/users/:id", get(api::users::get_user))
            .route("/", get(api::root::get))
            .route("/error", get(api::root::error))
            .route_layer(middleware::from_fn(middlewares::require_authenticated_middleware::require_auth))
            .route("/users", post(api::users::create_user))
            .route_layer(middleware::from_fn_with_state(state, middlewares::azure_auth_middleware::azure_auth_middleware))
            .layer(TraceLayer::new_for_http())
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

fn get_az_auth() -> AzureAuth {
    let aud = std::env::var("AUD").expect("Could not read AUD from environment variable");
    AzureAuth::new(aud).expect("AUD not valid")
}
