use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector};

use crate::element_ref_extensions::Extensions;
use crate::{Article, ArticleInstruction, BlogError, BlogIndex, Order, ParseInstruction};

use self::regex_parser::{regex_parse, regex_parse_single_node};

mod blog_index_tests;
mod regex_parser;

static SELECT_ALL: Lazy<Selector> = Lazy::new(|| Selector::parse("*").unwrap());

pub fn parse_links(blog: &BlogIndex, html: &str) -> Result<Vec<String>, BlogError> {
    let document = Html::parse_document(html);
    let content_node = get_content_node(&document, &blog.section)?;

    match &blog.link_selector {
        ParseInstruction::Selectors(selector, order) => {
            let outer_selector =
                Selector::parse(selector).map_err(|_| BlogError::SelectorError(selector.to_string()))?;
            let a_selector = Selector::parse("a").map_err(|_| BlogError::SelectorError("a".to_string()))?;
            let selects = content_node.select(&outer_selector);

            let links = selects
                .filter_map(|select| get_ordered_element_ref(select, &a_selector, order))
                .filter_map(Extensions::get_url)
                .collect::<Vec<String>>();

            Ok(links)
        }
        ParseInstruction::Regex(re) => Ok(regex_parse(&content_node.html(), re)?),
    }
}

pub fn parse_article(blog: &ArticleInstruction, html: &str) -> Result<Article, BlogError> {
    let document = Html::parse_document(html);
    let content_node = get_content_node(&document, &blog.section)?;

    let headline = match &blog.headline {
        ParseInstruction::Selectors(selector, order) => {
            get_ordered_element_ref_from_string(content_node, selector, order)?.get_string()
        }
        ParseInstruction::Regex(x) => regex_parse_single_node(&content_node.html(), x)?,
    };

    let content = match &blog.content {
        Some(content) => match content {
            ParseInstruction::Selectors(selector, order) => {
                get_ordered_element_ref_from_string(content_node, selector, order)?.html()
            }
            ParseInstruction::Regex(x) => regex_parse_single_node(&content_node.html(), x)?,
        },
        None => content_node.html(),
    };

    let date = match &blog.date {
        Some(content) => {
            let date = match content {
                ParseInstruction::Selectors(selector, order) => {
                    get_ordered_element_ref_from_string(content_node, selector, order)?.get_string()
                }
                ParseInstruction::Regex(x) => {
                    let document = Html::parse_fragment(&regex_parse_single_node(&content_node.html(), x)?);
                    document
                        .select(&SELECT_ALL)
                        .next()
                        .ok_or(BlogError::Generic("Could not select *".to_string()))?
                        .get_string()
                }
            };
            Some(date)
        }
        None => None,
    };

    Ok(Article {
        headline,
        content,
        date,
    })
}

fn get_content_node<'a>(document: &'a Html, instruction: &ParseInstruction) -> Result<ElementRef<'a>, BlogError> {
    let selector = Selector::parse("html").map_err(|_| BlogError::SelectorError("html".to_string()))?;
    let root_node = document
        .select(&selector)
        .next()
        .ok_or(BlogError::Generic("Could not find html node".to_string()))?;

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
    let s = Selector::parse(selector).map_err(|_| BlogError::SelectorError(selector.to_string()))?;
    get_ordered_element_ref(node, &s, order)
        .ok_or(BlogError::OrderedElementNotFound(selector.clone(), order.clone()))
}

fn get_ordered_element_ref<'a>(node: ElementRef<'a>, selector: &Selector, order: &Order) -> Option<ElementRef<'a>> {
    let mut select = node.select(selector);
    match order {
        Order::Normal(n) => select.nth(*n),
        Order::Reverse(n) => select.collect::<Vec<_>>().into_iter().rev().nth(*n),
    }
}
