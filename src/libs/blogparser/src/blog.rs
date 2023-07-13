use scraper::{ElementRef, Html, Selector};

use crate::blog_parser::{Article, Blog, Order, ParseInstruction};
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

        result.text().await.expect("Could not convert response to html")
    }

    pub fn parse_links(self: &Blog, html: &String) -> Vec<String> {
        let document = Html::parse_document(html);
        let content_node = get_content_node(&document, &self.index.section);

        match &self.index.link_selector {
            ParseInstruction::Selectors(selector, order) => {
                let outer_selector = Selector::parse(&selector).expect(&format!("Could not parsej{}", selector));
                let a_selector = Selector::parse("a").expect(&format!("Could not parsej{}", "a"));
                let selects = content_node.select(&outer_selector);

                selects
                    .map(|select| get_ordered_element_ref(select, &a_selector, order))
                    .filter_map(|x| x)
                    .filter_map(|x| x.get_url())
                    .collect::<Vec<String>>()
            }
            ParseInstruction::Regexp(_) => todo!(),
        }
    }

    pub fn parse_article(self: &Blog, html: &String) -> Article {
        let document = Html::parse_document(html);
        let content_node = get_content_node(&document, &self.article.section);

        let headline = get_element_ref(content_node, &self.article.headline).unwrap();

        let content = match &self.article.content {
            Some(content) => get_element_ref(content_node, &content).unwrap(),
            None => content_node,
        };

        let date = match &self.article.date {
            Some(content) => Some(get_element_ref(content_node, content).unwrap().get_string()),
            None => None,
        };

        Article {
            headline: headline.get_string(),
            content: content.html(),
            date: date,
        }
    }
}

fn get_content_node<'a>(document: &'a Html, instruction: &ParseInstruction) -> ElementRef<'a> {
    let root_node = document.select(&Selector::parse("html").unwrap()).next().unwrap();
    get_element_ref(root_node, instruction).unwrap()
}

fn get_element_ref<'a>(node: ElementRef<'a>, instruction: &ParseInstruction) -> Option<ElementRef<'a>> {
    match instruction {
        ParseInstruction::Selectors(selector, order) => {
            let selector = Selector::parse(&selector).expect(&format!("Could not parse {}", selector));
            get_ordered_element_ref(node, &selector, &order)
        }
        ParseInstruction::Regexp(x) => todo!(),
    }
}

fn get_ordered_element_ref<'a>(node: ElementRef<'a>, selector: &Selector, order: &Order) -> Option<ElementRef<'a>> {
    let select = node.select(&selector);
    match order {
        Order::Normal(n) => select.skip(*n).next(),
        Order::Reverse(n) => select.collect::<Vec<_>>().into_iter().rev().skip(*n).next(),
    }
}
