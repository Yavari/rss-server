use serde::Deserialize;

#[derive(Deserialize)]
pub struct Instructions {
    pub json: String,
}
