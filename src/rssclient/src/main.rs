use blog::Blog;
use futures::future::join_all;
use parser::{
    ArticleParseInstructions, BlogParseInstructions, Order, ParseInstruction,
    SingleParseInstruction,
};
use reqwest::Client;

mod blog;
mod parser;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let blog = Blog {
        url: "https://smallcultfollowing.com".to_string(),
        suffix: Some("/babysteps".to_string()),
        blog_instructions: BlogParseInstructions {
            section: ParseInstruction::Class("content_body".to_string(), Order::Normal(0)),
            urls: ParseInstruction::Class("post_section".to_string(), Order::Reverse(0)),
        },
        article_instructions: ArticleParseInstructions {
            article: ParseInstruction::Tag("article".to_string(), Order::Normal(0)),
            title: ParseInstruction::Tag("h1".to_string(), Order::Normal(0)),
            date: Some(ParseInstruction::Class(
                "post-meta".to_string(),
                parser::Order::Normal(0),
            )),
            content: Some(ParseInstruction::Class("post-content".to_string(), Order::Normal(0))),
        },
    };

    fetch(&client, blog).await;

    let blog = Blog {
        url: "https://payam.yavari.se".to_string(),
        suffix: None,
        blog_instructions: BlogParseInstructions {
            section: ParseInstruction::Class("col-md-10".to_string(), Order::Normal(0)),
            urls: ParseInstruction::Tag("li".to_string(), Order::Reverse(0)),
        },
        article_instructions: ArticleParseInstructions {
            article: ParseInstruction::Class("content".to_string(), Order::Normal(0)),
            title: ParseInstruction::Tag("h1".to_string(), Order::Normal(0)),
            date: None,
            content: None,
        },
    };

    fetch(&client, blog).await;
}

async fn fetch(client: &Client, blog: Blog) {
    let (urls, errors): (Vec<_>, Vec<_>) = blog
        .get_blog(&client)
        .await
        .into_iter()
        .partition(Result::is_ok);

    for error in errors {
        print!("Error{}", error.unwrap_err())
    }

    let articles = urls
        .into_iter()
        .map(Result::unwrap)
        .map(|u| blog.get_article(&client, u));
    let results = join_all(articles).await;

    for article in results {
        if let Ok(article) = article {
            let date = if let Some(d) = article.date {
                d
            } else {
                "".to_string()
            };
            println!("{}\t{}\t{}", date, article.link, article.title);

            if article.title == "Setup Dropbox in Ubuntu" {
                println!("{}", article.description);
            }
        } else if let Err(e) = article {
            println!("error {}", e);
        }
    }
}
