use crate::{Blog, BlogError};
use reqwest::Client;

impl Blog {
    pub async fn fetch_blog(&self, client: &Client) -> Result<String, BlogError> {
        self.fetch_url(&self.blog_url(), client).await
    }

    pub async fn fetch_article(&self, url: &str, client: &Client) -> Result<String, BlogError> {
        self.fetch_url(&self.article_url(url), client).await
    }

    async fn fetch_url(&self, url: &str, client: &Client) -> Result<String, BlogError> {
        println!("{}", url);
        let result = client
            .get(url)
            .send()
            .await
            .map_err(|x| BlogError::Generic(format!("Could not fetch {}. {}", url, x)))?;
        result.text().await.map_err(|x| BlogError::Generic(format!("Could read text {}. {}", url, x)))
    }

    fn blog_url(&self) -> String {
        match &self.url_suffix {
            Some(suffix) => format!("{}/{}", &self.url, &suffix),
            None => self.url.clone(),
        }
    }

    pub fn article_url(&self, url: &str) -> String {
        format!("{}{}", self.url, url)
    }
}
