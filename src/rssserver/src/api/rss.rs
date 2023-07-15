use axum::{extract::Path, response::Html};
use blogparser::{
    blog_client::{BlogClient, HttpClient},
    blog_parser::{ArticleInstruction, Blog, BlogIndex, Order, ParseInstruction},
};
use reqwest::Client;

pub async fn view() -> Html<String> {
    let blogs = get_blogs()
        .iter()
        .enumerate()
        .map(|(i, b)| format!("<a href='/rss/blogs/{}'>{}</a>", i, b.url))
        .collect::<Vec<String>>()
        .join("<br/>");
    Html(blogs)
}

pub async fn view_blog(Path(id): Path<usize>) -> Html<String> {
    let client = Box::new(HttpClient::new(Client::new())) as Box<dyn BlogClient>;
    let blog = get_blog(id);
    if let Some(blog) = blog {
        let response = blog.fetch_blog(&client).await;
        let urls = blog.parse_links(&response);
        let urls = urls;
        if let Ok(urls) = urls {
            let a = urls
                .iter()
                .filter_map(|x| x.as_ref().ok())
                .map(|f| format!("<a href='/rss/blogs/{}/articles{}'>{}</a>", id, f, f))
                .collect::<Vec<String>>()
                .join("<br/>");

            return Html(a);
        } else {
        }
        return Html("Error".to_string());
    } else {
        return Html("Not found".to_string());
    }
}

pub async fn view_article(Path((id, path)): Path<(usize, String)>) -> Html<String> {
    let client = Box::new(HttpClient::new(Client::new())) as Box<dyn BlogClient>;
    println!("{}", path);
    let blog = get_blog(id);
    if let Some(blog) = blog {
        let url = "/".to_string() + &path;
        let html = blog.fetch_article(&url, &client).await;
        let article = blog.parse_article(&html);

        if let Ok(article) = article {
            let html = if let Some(date) = article.date {
                format!("<h1>{}</h1><p>{}</p><hr/>{}", article.headline, date, article.content)
            } else {
                format!("<h1>{}</h1><hr/>{}", article.headline, article.content)
            };

            return Html(html);
        } else {
            return Html("ERROR".to_string());
        }
    } else {
        return Html("Not found".to_string());
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
    get_blogs().iter().skip(index).next().map(|f| (*f).clone())
}
