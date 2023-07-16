pub mod blog_encoding;
pub mod blog_getter;
pub mod element_ref_extensions;
pub mod regex_parser;
use scraper::Selector;
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
    OrderedElementNotFound(Selector, Order),
    FromJsonParseError(serde_json::Error, String),
}
