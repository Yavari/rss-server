pub mod article_content_parser;
pub mod blog_content_parser;
mod node_parser;
pub struct Article {
    pub title: String,
    pub link: String,
    pub date: Option<String>,
    pub description: String,
}
pub struct ArticleParseInstructions {
    pub article: ParseInstruction,
    pub title: ParseInstruction,
    pub date: Option<ParseInstruction>,
    pub content: Option<ParseInstruction>,
}

pub struct BlogParseInstructions {
    pub section: ParseInstruction,
    pub urls: ParseInstruction,
}

pub enum ParseInstruction {
    Tag(String, Order),
    Class(String, Order),
}

pub enum SingleParseInstruction {
    Tag(String),
    Class(String),
}

#[derive(Clone)]
pub enum Order {
    Normal(usize),
    Reverse(usize)
}