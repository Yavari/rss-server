use crate::{
    models::{Instructions, XmlError},
    response::Xml,
    AppState,
};
use axum::{
    extract::{Query, State},
    Json,
};
use axum_macros::debug_handler;
use blogparser::{
    Blog, {parse_article, parse_links},
};
use reqwest::Client;
use rss::{Channel, ChannelBuilder, ItemBuilder};
use std::error::Error;
use tracing::error;

#[debug_handler]
pub async fn get(state: State<AppState>, Query(query): Query<Instructions>) -> Result<Xml<String>, Xml<String>> {
    match get_blog(&state.client, query).await {
        Ok(channel) => Ok(Xml(channel.to_string())),
        Err(e) => Err(XmlError::create("Something went wrong", e).get_response()),
    }
}

pub async fn error() -> Json<Message> {
    panic!("This is a test")
}

async fn get_blog(client: &Client, query: Instructions) -> Result<Channel, Box<dyn Error>> {
    let blog = Blog::from_json(&query.json)?;
    let response = blog.fetch_blog(&client).await;
    let links = parse_links(&blog.index, &response)?;

    for l in (&links).into_iter().filter_map(|x| x.as_ref().err()) {
        error!("Could not parse link: {}", l);
    }

    let items = (&links)
        .into_iter()
        .filter_map(|x| x.as_ref().ok())
        .map(|url| get_article(&client, &blog, url));
    let items = futures::future::join_all(items).await;

    let channel = ChannelBuilder::default()
        .title(&blog.title.to_string())
        .link(&blog.url.to_string())
        .items(items)
        .build();

    return Ok(channel);
}

async fn get_article(client: &Client, blog: &Blog, url: &str) -> rss::Item {
    let html = blog.fetch_article(&url, client).await;
    let article = parse_article(&blog.article, &html);
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
