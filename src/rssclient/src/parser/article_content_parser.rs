use select::{document::Document, predicate::Name};

use super::{
    node_parser::{get_node, get_node_as_text},
    Article, ArticleParseInstructions,
};

pub fn get_content_from_html(
    url: &str,
    html: &str,
    instruction: &ArticleParseInstructions,
) -> Result<Article, String> {
    let document = Document::from(html);
    let root_node = get_node(
        &instruction.article,
        document
            .find(Name("html"))
            .next()
            .ok_or(format!("Could not find <html> in article: {}", url))?,
        html,
    )?;

    let date = if let Some(date) = &instruction.date {
        Some(get_node_as_text(date, root_node, html)?)
    } else {
        None
    };

    let description = if let Some(instruction) = &instruction.content {
        get_node_as_text(instruction, root_node, html)?
    } else {
        root_node.text()
    };

    let article = Article {
        title: get_node_as_text(&instruction.title, root_node, html)?,
        link: url.to_string(),
        date: date,
        description: description,
    };

    Ok(article)
}
