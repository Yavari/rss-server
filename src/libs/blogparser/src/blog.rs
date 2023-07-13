use once_cell::sync::Lazy;
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};

use crate::blog_parser::{Article, Blog, Order, ParseInstruction};
use crate::element_ref_extensions::Extensions;
use crate::regex_parser::RegexParser;

static SELECT_ALL: Lazy<Selector> = Lazy::new(|| Selector::parse("*").unwrap());

impl Blog {
    fn get_blog_url(self: &Blog) -> String {
        match &self.url_suffix {
            Some(suffix) => format!("{}/{}", &self.url, &suffix),
            None => self.url.clone(),
        }
    }

    pub async fn fetch_blog(self: &Blog, client: &Client) -> String {
        self.fetch_url(&self.get_blog_url(), client).await
    }

    pub async fn fetch_article(self: &Blog, url: &str, client: &Client) -> String {
        self.fetch_url(&format!("{}{}", &self.url, url), client).await
    }

    async fn fetch_url(self: &Blog, url: &str, client: &Client) -> String {
        println!("{}", url);
        let result = client.get(url).send().await.expect(&format!("Could not fetch {}", url));

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
            ParseInstruction::Regex(re) => {
                let regex_parser = RegexParser::create_vec(&content_node.html(), re);
                regex_parser.into_iter().map(|x| x.html().to_string()).collect()
            }
        }
    }

    pub fn parse_article(self: &Blog, html: &String) -> Article {
        let document = Html::parse_document(html);
        let content_node = get_content_node(&document, &self.article.section);

        let headline = match &self.article.headline {
            ParseInstruction::Selectors(selector, order) => {
                get_ordered_element_ref_from_string(content_node, &selector, &order)
                    .unwrap()
                    .get_string()
            }
            ParseInstruction::Regex(x) => RegexParser::create(&content_node.html(), x).html().to_string(),
        };

        let content = match &self.article.content {
            Some(content) => match content {
                ParseInstruction::Selectors(selector, order) => {
                    get_ordered_element_ref_from_string(content_node, &selector, &order)
                        .unwrap()
                        .html()
                }
                ParseInstruction::Regex(x) => RegexParser::create(&content_node.html(), x).html().to_string(),
            },
            None => content_node.html(),
        };

        let date = match &self.article.date {
            Some(content) => match content {
                ParseInstruction::Selectors(selector, order) => Some(
                    get_ordered_element_ref_from_string(content_node, &selector, &order)
                        .unwrap()
                        .get_string(),
                ),
                ParseInstruction::Regex(x) => {
                    let parser = RegexParser::create(&content_node.html(), x);
                    let fragment = parser.html();
                    let document = Html::parse_fragment(fragment);
                    Some(document.select(&SELECT_ALL).next().unwrap().get_string())
                }
            },
            None => None,
        };

        Article {
            headline: headline,
            content: content,
            date: date,
        }
    }
}

fn get_content_node<'a>(document: &'a Html, instruction: &ParseInstruction) -> ElementRef<'a> {
    let root_node = document.select(&Selector::parse("html").unwrap()).next().unwrap();
    match instruction {
        ParseInstruction::Selectors(selector, order) => {
            get_ordered_element_ref_from_string(root_node, selector, order).unwrap()
        }
        ParseInstruction::Regex(_) => todo!(),
    }
}

fn get_ordered_element_ref_from_string<'a>(
    node: ElementRef<'a>,
    selector: &String,
    order: &Order,
) -> Option<ElementRef<'a>> {
    let selector = Selector::parse(&selector).expect(&format!("Could not parse {}", selector));
    get_ordered_element_ref(node, &selector, &order)
}

fn get_ordered_element_ref<'a>(node: ElementRef<'a>, selector: &Selector, order: &Order) -> Option<ElementRef<'a>> {
    let select = node.select(&selector);
    match order {
        Order::Normal(n) => select.skip(*n).next(),
        Order::Reverse(n) => select.collect::<Vec<_>>().into_iter().rev().skip(*n).next(),
    }
}
