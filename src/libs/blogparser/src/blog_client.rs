use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait BlogClient: Send + Sync {
    async fn get_text(&self, path: &str) -> String;
}

pub struct FileClient {}

pub struct HttpClient {
    client: Client,
}

#[async_trait]
impl BlogClient for FileClient {
    async fn get_text(&self, str: &str) -> String {
        todo!()
    }
}

impl HttpClient {
    pub fn new(client: Client) -> HttpClient {
        HttpClient { client: client }
    }
}

#[async_trait]
impl BlogClient for HttpClient {
    async fn get_text(&self, path: &str) -> String {
        let result = self
            .client
            .get(path)
            .send()
            .await
            .expect(&format!("Could not fetch {}", path));
        result.text().await.expect("Could not convert response to html")
    }
}
