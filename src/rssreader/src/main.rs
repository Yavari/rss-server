use blogparser::{blog_parser::{ArticleInstruction, Blog, BlogIndex, Order, ParseInstruction}, blog::Error};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let client: Client = Client::new();

    let blogs = vec![
        Blog {
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
                date: None,
            },
        },
        Blog {
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
    ];

    for blog in &blogs {
        let str = blog.to_html_safe_string();
        println!("{}", str);
    }

    for blog in &blogs {
        let str = &blog.to_html_safe_string();
        let blog = Blog::from_html_safe_string(str);
        let response = blog.fetch_blog(&client).await;
        let urls = blog.parse_links(&response)?;
        println!("{:?}", urls);
        for url in urls {
            let html = blog.fetch_article(&url, &client).await;
            println!("{:?}", blog.parse_article(&html));
            return Ok(());
        }
    }

    Ok(())
}
