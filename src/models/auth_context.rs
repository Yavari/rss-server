#[derive(serde::Serialize, Clone, Debug, PartialEq)]
pub struct AuthContext{
    pub is_signed_in: bool,
    pub azp: Option<String>,
}