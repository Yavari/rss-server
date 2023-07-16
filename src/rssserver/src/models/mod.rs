use std::error::Error;

use reqwest::Client;
use serde::Deserialize;
use tracing::error;

use crate::response::Xml;

#[derive(Deserialize)]
pub struct Instructions {
    pub json: String,
}

pub struct XmlError {
    message: String,
}

impl XmlError {
    pub fn create(message: &str, e: Box<dyn Error>) -> XmlError {
        let e = format!("Something went wrong.\n{}", e);
        error!(e);
        XmlError {
            message: message.to_owned(),
        }
    }
    pub fn get_response(&self) -> Xml<String> {
        Xml(format!("<error>{}</error>",self.message))
    }
}

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
}
