use azure_jwt::AzureAuth;

#[derive(Clone)]
pub struct AppState {
    pub azure_auth: AzureAuth,
}
