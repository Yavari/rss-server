use reqwest::Client;
use crate::Blog;

impl Blog {
    pub async fn fetch_blog(&self, client: &Client) -> String {
        self.fetch_url(&self.get_blog_url(), client).await
    }

    async fn fetch_url(&self, url: &str, client: &Client) -> String {
        println!("{}", url);
        let result = client
            .get(url)
            .send()
            .await
            .expect(&format!("Could not fetch {}", url));
        result.text().await.expect("Could not convert response to html")
    }

    pub async fn fetch_article(&self, url: &str, client: &Client) -> String {
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
