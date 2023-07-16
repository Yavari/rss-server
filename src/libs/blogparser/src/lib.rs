#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]


pub mod blog_encoding;
pub mod blog_getter;
pub mod element_ref_extensions;
use core::fmt;

use serde::{Deserialize, Serialize};

mod blog_parser;


pub use blog_parser::parse_article as parse_article;
pub use blog_parser::parse_links as parse_links;


#[derive(Clone, Serialize, Deserialize)]
pub struct Blog {
    pub title: String,
    pub url: String,
    pub url_suffix: Option<String>,
    pub index: BlogIndex,
    pub article: ArticleInstruction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub headline: String,
    pub content: String,
    pub date: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BlogIndex {
    pub section: ParseInstruction,
    pub link_selector: ParseInstruction,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ArticleInstruction {
    pub section: ParseInstruction,
    pub headline: ParseInstruction,
    pub date: Option<ParseInstruction>,
    pub content: Option<ParseInstruction>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ParseInstruction {
    Selectors(String, Order),
    Regex(String),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Order {
    Normal(usize),
    Reverse(usize),
}

#[derive(Debug)]
pub enum BlogError {
    OrderedElementNotFound(String, Order),
    FromJsonParseError(serde_json::Error, String),
    SelectorError(String),
    Generic(String),
    RegEx(regex::Error)
}

impl std::error::Error for BlogError {}

impl fmt::Display for BlogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::OrderedElementNotFound(selector, order) => {
                write!(f, "Could not find ordered element {selector:?} {order:?}")
            }
            Self::FromJsonParseError(e, json) => {
                write!(f, "Could not parse json with error message: {e}\n Json: {json}")
            }

            Self::SelectorError(selector) => write!(f, "Could not parse selector {selector}"),
            Self::Generic(message) => write!(f, "{message}"),
            Self::RegEx(x) => write!(f, "Could not parse regex: {x}"),
        }
    }
}