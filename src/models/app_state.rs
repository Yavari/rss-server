use std::sync::{Arc, Mutex};

use azure_jwt::AzureAuth;

#[derive(Clone)]
pub struct AppState {
    pub azure_auth: Arc<Mutex<AzureAuth>>,
}
