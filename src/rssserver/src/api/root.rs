use std::error::Error;
use tracing::error;
use crate::{
    models::{Instructions, XmlError},
    response::Xml,
};
use axum::{extract::Query, Json};
use blogparser::blog_parser::Blog;
use reqwest::Client;
use rss::{Channel, ChannelBuilder, ItemBuilder};

pub async fn get(Query(query): Query<Instructions>) -> Result<Xml<String>, Xml<String>> {
    match get_blog(query).await {
        Ok(channel) => Ok(Xml(channel.to_string())),
        Err(e) => Err(XmlError::create("Something went wrong", e).get_response()),
    }
}

pub async fn error() -> Json<Message> {
    panic!("This is a test")
}

async fn get_blog(query: Instructions) -> Result<Channel, Box<dyn Error>> {
    let client: Client = Client::new();
    let blog = Blog::from_json(&query.json)?;
    let response = blog.fetch_blog(&client).await;
    let links = &blog.parse_links(&response)?;

    for l in links.into_iter().filter_map(|x| x.as_ref().err()) {
        error!("Could not parse link: {}", l);
    }

    let items = links
        .into_iter()
        .filter_map(|x| x.as_ref().ok())
        .map(|url| get_article(&blog, url, &client));
    let items = futures::future::join_all(items).await;

    let channel = ChannelBuilder::default()
        .title(&blog.title.to_string())
        .link(&blog.url.to_string())
        .items(items)
        .build();

    return Ok(channel);
}

async fn get_article(blog: &Blog, url: &str, client: &Client) -> rss::Item {
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
