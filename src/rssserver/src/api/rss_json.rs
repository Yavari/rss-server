use axum::{
    extract::{Path, Query},
    response::Html,
};
use blogparser::blog_parser::Blog;
use reqwest::Client;
use rss::{ChannelBuilder, ItemBuilder};
use serde::Deserialize;

use crate::response::Xml;

pub async fn view_blog(query: Query<Instructions>) -> Html<String> {
    let client: Client = Client::new();
    println!("{:?}", query.json);
    let blog = Blog::from_json(&query.json);
    let response = blog.fetch_blog(&client).await;
    let urls = blog.parse_links(&response);
    let a = urls
        .iter()
        .map(|f| format!("<a href='/rss_json/blogs/articles{}?json={}'>{}</a>", f, query.json, f))
        .collect::<Vec<String>>()
        .join("<br/>");

    return Html(a);
}

pub async fn view_article(Path(path): Path<String>, Query(query): Query<Instructions>) -> Html<String> {
    let client: Client = Client::new();
    println!("{}", path);
    let blog = Blog::from_json(&query.json);
    let url = "/".to_string() + &path;
    let html = blog.fetch_article(&url, &client).await;
    let article = blog.parse_article(&html);

    let html = if let Some(date) = article.date {
        format!("<h1>{}</h1><p>{}</p><hr/>{}", article.headline, date, article.content)
    } else {
        format!("<h1>{}</h1><hr/>{}", article.headline, article.content)
    };

    return Html(html);
}

pub async fn view(Query(query): Query<Instructions>) -> Xml<String> {
    let client: Client = Client::new();
    let blog = Blog::from_json(&query.json);
    let response = blog.fetch_blog(&client).await;
    let items = blog
        .parse_links(&response)
        .into_iter()
        .map(|url| get_article(&query, url, &client));
    let items = futures::future::join_all(items).await;

    let channel = ChannelBuilder::default()
        .title("Channel Title".to_string())
        .link(&blog.url.to_string())
        .description("An RSS feed.".to_string())
        .items(items)
        .build();

    return Xml(channel.to_string());
}

async fn get_article(query: &Instructions, url: String, client: &Client) -> rss::Item {
    let blog = Blog::from_json(&query.json);
    let url = "/".to_string() + &url;
    let html = blog.fetch_article(&url, client).await;
    let article = blog.parse_article(&html);

    ItemBuilder::default()
        .title(Some(article.headline))
        .pub_date(article.date)
        .content(Some(article.content))
        .build()
}

#[derive(Deserialize)]
pub struct Instructions {
    json: String,
}
