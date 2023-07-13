pub mod blog;
pub mod element_ref_extensions;

pub mod blog_parser {
    use reqwest::Client;

    #[derive(Clone)]
    pub struct Blog {
        pub client: Client,
        pub url: String,
        pub url_suffix: Option<String>,
        pub index: BlogIndex,
        pub article: ArticleInstruction,
    }

    #[derive(Debug, Clone)]
    pub struct Article {
        pub headline: String,
        pub content: String,
        pub date: Option<String>,
    }

    #[derive(Clone)]
    pub struct BlogIndex {
        pub section: ParseInstruction,
        pub link_selector: ParseInstruction,
    }

    #[derive(Clone)]
    pub struct ArticleInstruction {
        pub section: ParseInstruction,
        pub headline: ParseInstruction,
        pub date: Option<ParseInstruction>,
        pub content: Option<ParseInstruction>,
    }

    #[derive(Clone)]
    pub enum ParseInstruction {
        Selectors(String, Order),
        Regexp(String),
    }

    #[derive(Clone)]
    pub enum Order {
        Normal(usize),
        Reverse(usize),
    }
}
