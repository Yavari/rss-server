use std::fmt;

use once_cell::sync::Lazy;
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};

use crate::blog_client::BlogClient;
use crate::blog_parser::{Article, Blog, Order, ParseInstruction};
use crate::element_ref_extensions::Extensions;
use crate::regex_parser::RegexParser;

static SELECT_ALL: Lazy<Selector> = Lazy::new(|| Selector::parse("*").unwrap());

impl Blog {
    fn get_blog_url(&self) -> String {
        match &self.url_suffix {
            Some(suffix) => format!("{}/{}", &self.url, &suffix),
            None => self.url.clone(),
        }
    }

    pub async fn fetch_blog(&self, client: &Box<dyn BlogClient>) -> String {
        self.fetch_url(&self.get_blog_url(), client).await
    }

    pub async fn fetch_article(&self, url: &str, client: &Box<dyn BlogClient>) -> String {
        self.fetch_url(&self.article_url(url), client).await
    }

    pub fn article_url(&self, url: &str) -> String {
        format!("{}{}", self.url, url)
    }

    async fn fetch_url(&self, url: &str, client: &Box<dyn BlogClient>) -> String {
        println!("{}", url);
        client.get_text(url).await
    }

    pub fn parse_links(&self, html: &String) -> Result<Vec<Result<String, BlogError>>, BlogError> {
        let document = Html::parse_document(html);
        let content_node = get_content_node(&document, &self.index.section)?;

        match &self.index.link_selector {
            ParseInstruction::Selectors(selector, order) => {
                let outer_selector = Selector::parse(&selector).expect(&format!("Could not parsej{}", selector));
                let a_selector = Selector::parse("a").expect(&format!("Could not parsej{}", "a"));
                let selects = content_node.select(&outer_selector);

                let links = selects
                    .map(|select| get_ordered_element_ref(select, &a_selector, order))
                    .map(|x| x.and_then(|y| Ok(y.get_url().unwrap())))
                    .collect::<Vec<Result<String, BlogError>>>();

                Ok(links)
            }
            ParseInstruction::Regex(re) => {
                let regex_parser = RegexParser::create_vec(&content_node.html(), re);
                let links = regex_parser.into_iter().map(|x|Ok(x.html().to_string())).collect();
                Ok(links)
            }
        }
    }

    pub fn parse_article(self: &Blog, html: &String) -> Result<Article, BlogError> {
        let document = Html::parse_document(html);
        let content_node = get_content_node(&document, &self.article.section)?;

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

        Ok(Article {
            headline: headline,
            content: content,
            date: date,
        })
    }
}

fn get_content_node<'a>(document: &'a Html, instruction: &ParseInstruction) -> Result<ElementRef<'a>, BlogError> {
    let root_node = document.select(&Selector::parse("html").unwrap()).next().unwrap();
    match instruction {
        ParseInstruction::Selectors(selector, order) => get_ordered_element_ref_from_string(root_node, selector, order),
        ParseInstruction::Regex(_) => todo!(),
    }
}

fn get_ordered_element_ref_from_string<'a>(
    node: ElementRef<'a>,
    selector: &String,
    order: &Order,
) -> Result<ElementRef<'a>, BlogError> {
    let s = Selector::parse(&selector).expect(&format!("Could not parse {}", selector));
    get_ordered_element_ref(node, &s, &order)
}

fn get_ordered_element_ref<'a>(
    node: ElementRef<'a>,
    selector: &Selector,
    order: &Order,
) -> Result<ElementRef<'a>, BlogError> {
    let select = node.select(&selector);
    let result = match order {
        Order::Normal(n) => select.skip(*n).next(),
        Order::Reverse(n) => select.collect::<Vec<_>>().into_iter().rev().skip(*n).next(),
    };

    result.ok_or(BlogError::OrderedElementNotFound(selector.to_owned(), order.clone()))
}

#[derive(Debug)]
pub enum BlogError {
    OrderedElementNotFound(Selector, Order),
    FromJsonParseError(serde_json::Error, String),
}

impl std::error::Error for BlogError {}

impl fmt::Display for BlogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlogError::OrderedElementNotFound(selector, order) => write!(f, "Could not find ordered element {:?} {:?}", selector, order),
            BlogError::FromJsonParseError(e, json) => {
                write!(f, "Could not parse json with error message: {}\n Json: {}", e, json)
            }
        }
    }
}
