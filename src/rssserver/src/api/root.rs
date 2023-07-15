use axum::{
    Json, extract::Query
};
use blogparser::blog_parser::Blog;
use reqwest::Client;
use rss::{ChannelBuilder, ItemBuilder};

use crate::{response::Xml, models::Instructions};

pub async fn get(Query(query): Query<Instructions>) -> Xml<String> {
    let client: Client = Client::new();
    let blog = Blog::from_json(&query.json);
    let response = blog.fetch_blog(&client).await;
    let links = blog.parse_links(&response);
    if let Ok(links) = links {
        let items = links.into_iter().map(|url| get_article(&query, url, &client));
        let items = futures::future::join_all(items).await;

        let channel = ChannelBuilder::default()
            .title(&blog.title.to_string())
            .link(&blog.url.to_string())
            .items(items)
            .build();

        return Xml(channel.to_string());
    } else {
        return Xml("Could not read index url".to_string());
    }
}


pub async fn error() -> Json<Message> {
    panic!("This is a test")
}



async fn get_article(query: &Instructions, url: String, client: &Client) -> rss::Item {
    let blog = Blog::from_json(&query.json);
    let html = blog.fetch_article(&url, client).await;
    let article = blog.parse_article(&html);
    if let Ok(article) = article {
        ItemBuilder::default()
            .title(Some(article.headline))
            .link(Some(blog.article_url(&url)))
            .pub_date(article.date)
            .content(Some(article.content))
            .build()
    } else {
        ItemBuilder::default()
            .title(Some("Eroor!".to_string()))
            .link(Some(blog.article_url(&url)))
            .content(Some("Could not parse".to_string()))
            .build()
    }
}

#[derive(serde::Serialize)]
pub struct Message {
    message: String,
}
