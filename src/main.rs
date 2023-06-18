mod api;

use axum::{
    routing::get,
    Router,
};
use std::{net::SocketAddr};



#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(api::root::handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
