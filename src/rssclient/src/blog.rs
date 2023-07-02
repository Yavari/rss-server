use reqwest::Client;

use crate::parser::{
    article_content_parser::get_content_from_html, blog_content_parser::get_article_urls, Article,
    ArticleParseInstructions, BlogParseInstructions,
};

pub struct Blog {
    pub url: String,
    pub suffix: Option<String>,
    pub blog_instructions: BlogParseInstructions,
    pub article_instructions: ArticleParseInstructions,
}

impl Blog {
    pub fn get_blog_url(&self) -> String {
        if let Some(suffix) = &self.suffix {
            format!("{}{}", self.url, suffix)
        } else {
            return self.url.clone();
        }
    }

    pub async fn get_article(&self, client: &Client, url: String) -> Result<Article, String> {
        let post_url = format!("{}{}", self.url, url);
        let result = client.get(&post_url).send().await.expect("error");
        let html = result.text().await.expect("msg");
        get_content_from_html(&post_url, &html, &self.article_instructions)
    }

    pub async fn get_blog(&self, client: &Client) -> Vec<Result<String, String>> {
        let result = client.get(self.get_blog_url()).send().await.expect("msg");
        let html = result.text().await.expect("msg");
        let result = get_article_urls(&html, &self.blog_instructions);
        if let Ok(result) = result{
            return result;
        }else{
            return vec![Err(result.unwrap_err())];
        }
    }
}
