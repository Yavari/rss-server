use axum::{
    extract::{Path, State},
    response::Html,
};
use blogparser::{
    {parse_article, parse_links}, {ArticleInstruction, Blog, BlogIndex, Order, ParseInstruction},
};

use crate::AppState;

pub async fn view() -> Html<String> {
    let blogs = get_blogs()
        .iter()
        .enumerate()
        .map(|(i, b)| format!("<a href='/rss/blogs/{}'>{}</a>", i, b.url))
        .collect::<Vec<String>>()
        .join("<br/>");
    Html(blogs)
}

pub async fn view_blog(state: State<AppState>, Path(id): Path<usize>) -> Html<String> {
    let blog = get_blog(id);
    if let Some(blog) = blog {
        let response = blog.fetch_blog(&state.client).await;
        if let Ok(response) = response {
            let urls = parse_links(&blog.index, &response);
            let urls = urls;
            if let Ok(urls) = urls {
                let a = urls
                    .iter()
                    .map(|f| format!("<a href='/rss/blogs/{}/articles{}'>{}</a>", id, f, f))
                    .collect::<Vec<String>>()
                    .join("<br/>");

                return Html(a);
            }
        }
        Html("Error".to_string())
    } else {
        Html("Not found".to_string())
    }
}

pub async fn view_article(state: State<AppState>, Path((id, path)): Path<(usize, String)>) -> Html<String> {
    println!("{}", path);
    let blog = get_blog(id);
    if let Some(blog) = blog {
        let url = "/".to_string() + &path;
        let html = blog.fetch_article(&url, &state.client).await;
        if let Ok(html) = html {
            let article = parse_article(&blog.article, &html);
            if let Ok(article) = article {
                let html = if let Some(date) = article.date {
                    format!("<h1>{}</h1><p>{}</p><hr/>{}", article.headline, date, article.content)
                } else {
                    format!("<h1>{}</h1><hr/>{}", article.headline, article.content)
                };

                return Html(html);
            }
        }
        Html("ERROR".to_string())
    } else {
        Html("Not found".to_string())
    }
}

fn get_blogs() -> Vec<Blog> {
    vec![
        Blog {
            title: "Small Cult Following".to_owned(),
            url: "https://smallcultfollowing.com".to_string(),
            url_suffix: Some("babysteps".to_string()),
            index: BlogIndex {
                section: ParseInstruction::Selectors(".content_body".to_string(), Order::Normal(0)),
                link_selector: ParseInstruction::Selectors(".post_section".to_string(), Order::Normal(0)),
            },
            article: ArticleInstruction {
                section: ParseInstruction::Selectors("article".to_string(), Order::Normal(0)),
                headline: ParseInstruction::Selectors("h1".to_string(), Order::Normal(0)),
                content: Some(ParseInstruction::Selectors(
                    ".post-content".to_string(),
                    Order::Normal(0),
                )),
                date: Some(ParseInstruction::Selectors("time".to_string(), Order::Reverse(0))),
            },
        },
        Blog {
            title: "Payam Yavari Blog".to_owned(),
            url: "https://payam.yavari.se".to_string(),
            url_suffix: None,
            index: BlogIndex {
                section: ParseInstruction::Selectors(".col-md-10".to_string(), Order::Normal(0)),
                link_selector: ParseInstruction::Selectors("li".to_string(), Order::Normal(0)),
            },
            article: ArticleInstruction {
                section: ParseInstruction::Selectors(".col-md-10".to_string(), Order::Normal(0)),
                headline: ParseInstruction::Selectors("h1".to_string(), Order::Normal(0)),
                content: None,
                date: Some(ParseInstruction::Regex(r"</h1>([\s\S]*?)<br>".to_string())),
            },
        },
        Blog {
            title: "Payam Yavari Blog".to_owned(),
            url: "https://payam.yavari.se".to_string(),
            url_suffix: None,
            index: BlogIndex {
                section: ParseInstruction::Selectors(".col-md-10".to_string(), Order::Normal(0)),
                link_selector: ParseInstruction::Regex(r#"<a href="(.*)">"#.to_string()),
            },
            article: ArticleInstruction {
                section: ParseInstruction::Selectors(".col-md-10".to_string(), Order::Normal(0)),
                headline: ParseInstruction::Selectors("h1".to_string(), Order::Normal(0)),
                content: None,
                date: Some(ParseInstruction::Regex(r"</h1>([\s\S]*?)<br>".to_string())),
            },
        },
    ]
}
fn get_blog(index: usize) -> Option<Blog> {
    get_blogs().get(index).map(|f| (*f).clone())
}
