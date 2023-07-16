use blogparser::{
    BlogError, {parse_article, parse_links}, {ArticleInstruction, Blog, BlogIndex, Order, ParseInstruction},
};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), BlogError> {
    let client = Client::new();
    let blogs = vec![
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
                date: None,
            },
        },
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
    ];

    for blog in &blogs {
        let str = blog.to_html_safe_string();
        println!("{}", str);
    }

    for blog in &blogs {
        let str = &blog.to_html_safe_string();
        let blog = Blog::from_html_safe_string(str);
        let response = blog.fetch_blog(&client).await;
        let urls = parse_links(&blog.index, &response)?;
        println!("{:?}", urls);

        for url in urls.into_iter().filter_map(|x| x.ok()) {
            let html = blog.fetch_article(&url, &client).await;
            println!("{:?}", parse_article(&blog.article, &html));
            return Ok(());
        }
    }

    Ok(())
}
