use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response, extract::State,
};
use tokio::task::spawn_blocking;
use tracing::{error, info, warn};

use crate::models::{auth_context::AuthContext, app_state::AppState};

pub async fn azure_auth_middleware<T>(
    State(state): State<AppState>,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    if let Some(token) = request.headers().typed_get::<Authorization<Bearer>>() {
        let token = token.token().to_owned();
        let decoded_token = spawn_blocking(move || state.azure_auth.lock().expect("").validate_token(&token))
            .await
            .map_err(|_| {
                error!("az_auth.validate_token task spawn failed");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        let auth_context = match decoded_token {
            Ok(decoded_token) => AuthContext {
                is_signed_in: true,
                azp: decoded_token.claims.azp,
            },
            Err(_) => {
                warn!("Token is not valid");
                AuthContext {
                    is_signed_in: false,
                    azp: None,
                }
            }
        };

        request.extensions_mut().insert(auth_context);

        Ok(next.run(request).await)
    } else {
        info!("Token not set");
        let auth_context = AuthContext {
            is_signed_in: false,
            azp: None,
        };
        request.extensions_mut().insert(auth_context);
        return Ok(next.run(request).await);
    }
}
