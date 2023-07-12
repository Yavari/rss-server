use scraper::{Html, Selector, ElementRef};

use crate::blog_parser::{Blog, ParseInstruction, Article, get_ordered_element_ref};
use crate::element_ref_extensions::Extensions;

impl Blog {
    fn get_blog_url(self: &Blog) -> String {
        match &self.url_suffix {
            Some(suffix) => format!("{}/{}", &self.url, &suffix),
            None => self.url.clone(),
        }
    }

    pub async fn fetch_blog(self: &Blog) -> String {
        self.fetch_url(&self.get_blog_url()).await
    }

    pub async fn fetch_article(self: &Blog, url: &str) -> String {
        self.fetch_url(&format!("{}{}", &self.url, url)).await
    }

    async fn fetch_url(self: &Blog, url: &str) -> String {
        println!("{}", url);
        let result = self
            .client
            .get(url)
            .send()
            .await
            .expect(&format!("Could not fetch {}", url));

        result
            .text()
            .await
            .expect("Could not convert response to html")
    }

    pub fn parse_links(self: &Blog, html: &String) -> Vec<String> {
        let document = Html::parse_document(html);
        let content_node = self.get_content_node(&self.index.section, &document);

        match &self.index.link_selector {
            ParseInstruction::Selectors(selector, order) => {
                let outer_selector =
                    Selector::parse(&selector).expect(&format!("Could not parsej{}", selector));
                let a_selector =
                    Selector::parse("a").expect(&format!("Could not parsej{}", "a"));
                let selects = content_node.select(&outer_selector);

                selects
                    .map(|select| get_ordered_element_ref(select, &a_selector, order))
                    .filter_map(|x| x)
                    .filter_map(|x| x.get_url())
                    .collect::<Vec<String>>()
            }
        }
    }

    pub fn parse_article(self: &Blog, html: &String) -> Article {
        let document = Html::parse_document(html);
        let content_node = self.get_content_node(&self.article.section, &document);

        let headline = match &self.article.headline {
            ParseInstruction::Selectors(selector, order) => {
                let selector =
                    Selector::parse(&selector).expect(&format!("Could not parsej{}", selector));
                get_ordered_element_ref(content_node, &selector, &order)
            }
            .unwrap(),
        };

        let content = match &self.article.content {
            Some(content) => match content {
                ParseInstruction::Selectors(selector, order) => {
                    let selector = Selector::parse(&selector)
                        .expect(&format!("Could not parsej{}", selector));
                    get_ordered_element_ref(content_node, &selector, &order)
                }
                .unwrap(),
            },
            None => content_node,
        };

        let date = match &self.article.date {
            Some(content) => match content {
                ParseInstruction::Selectors(selector, order) => {
                    let selector = Selector::parse(&selector)
                        .expect(&format!("Could not parsej{}", selector));
                    let node = get_ordered_element_ref(content_node, &selector, &order);

                    let result = node.unwrap().get_string();
                    Some(result)
                }
            },
            None => None,
        };

        Article {
            headline: headline.get_string(),
            content: content.html(),
            date: date,
        }
    }

    fn get_content_node<'a>(
        self: &Blog,
        instruction: &ParseInstruction,
        document: &'a Html,
    ) -> ElementRef<'a> {
        let root_node = document
            .select(&Selector::parse("html").unwrap())
            .next()
            .unwrap();
        match instruction {
            ParseInstruction::Selectors(selector, order) => {
                let selector =
                    Selector::parse(&selector).expect(&format!("Could not parsej{}", selector));
                get_ordered_element_ref(root_node, &selector, order)
            }
            .unwrap(),
        }
    }
}