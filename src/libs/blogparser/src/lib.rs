pub mod blog;
pub mod blog_encoding;
pub mod regex_parser;
pub mod element_ref_extensions;

pub mod blog_parser {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize)]
    pub struct Blog {
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
}
