use crate::{blog_client::BlogClient, Blog};

impl Blog {
    pub async fn fetch_blog(&self, client: &Box<dyn BlogClient>) -> String {
        self.fetch_url(&self.get_blog_url(), client).await
    }

    async fn fetch_url(&self, url: &str, client: &Box<dyn BlogClient>) -> String {
        println!("{}", url);
        client.get_text(url).await
    }

    pub async fn fetch_article(&self, url: &str, client: &Box<dyn BlogClient>) -> String {
        self.fetch_url(&self.article_url(url), client).await
    }

    fn get_blog_url(&self) -> String {
        match &self.url_suffix {
            Some(suffix) => format!("{}/{}", &self.url, &suffix),
            None => self.url.clone(),
        }
    }

    pub fn article_url(&self, url: &str) -> String {
        format!("{}{}", self.url, url)
    }
}
