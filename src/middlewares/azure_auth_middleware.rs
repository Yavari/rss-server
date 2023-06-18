use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use azure_jwt::AzureAuth;
use tokio::task::spawn_blocking;
use tracing::{error, info, warn};

use crate::models::auth_context::AuthContext;

pub async fn azure_auth_middleware<T>(
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    if let Some(token) = request.headers().typed_get::<Authorization<Bearer>>() {
        let mut az_auth = get_az_auth()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let token = token.token().to_owned();
        let decoded_token = spawn_blocking(move || az_auth.validate_token(&token))
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

// TODO: This should be passed in somehow
async fn get_az_auth() -> Result<AzureAuth, Box<dyn std::error::Error>> {
    let aud = std::env::var("AUD").map_err(|op| {
        error!("Could not read AUD from environment variable");
        op
    })?;
    let az_auth = spawn_blocking(|| AzureAuth::new(aud))
        .await
        .map_err(|op| {
            error!("AzureAuth::new task spawn failed");
            op
        })?
        .map_err(|op| {
            error!("AUD not valid");
            op
        })?;
    Ok(az_auth)
}
