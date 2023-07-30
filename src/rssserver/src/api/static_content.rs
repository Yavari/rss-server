use axum::{
    body::{self, Empty, Full},
    extract::Path,
    http::{HeaderValue, Response},
    response::IntoResponse,
};
use include_dir::{include_dir, Dir};
use reqwest::{header, StatusCode};
use tracing::info;

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

pub async fn static_root() -> impl IntoResponse {
    static_path(Path("index.html".to_string())).await
}

pub async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();
    match STATIC_DIR.get_file(path) {
        None => {
            info!("Could not find static file: {path}");
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body::boxed(Empty::new()))
                .unwrap()
        }
        Some(file) => {
            info!("Serving static file: {path} ({})", mime_type);
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, HeaderValue::from_str(mime_type.as_ref()).unwrap())
                .body(body::boxed(Full::from(file.contents())))
                .unwrap()
        }
    }
}
