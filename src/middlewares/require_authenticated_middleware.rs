use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response, Extension,
};

use crate::models::{auth_context::AuthContext};

pub async fn require_auth<T>(
    Extension(auth_context): Extension<AuthContext>,
    request: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    if !auth_context.is_signed_in {
        return Err(StatusCode::UNAUTHORIZED);
    }

    return Ok(next.run(request).await);
}
