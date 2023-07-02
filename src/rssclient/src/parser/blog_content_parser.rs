use select::{
    document::Document,
    node::Find,
    predicate::{Class, Name},
};

use super::{
    node_parser::{get_node, get_node_href},
    BlogParseInstructions, ParseInstruction, SingleParseInstruction,
};

pub fn get_article_urls(
    html: &str,
    instruction: &BlogParseInstructions,
) -> Result<Vec<Result<String, String>>, String> {
    let document = Document::from(html);
    let root_node = get_node(
        &instruction.section,
        document
            .find(Name("html"))
            .next()
            .ok_or(format!("Could not find <html> in blog"))?,
        html,
    )?;

    let a = match &instruction.urls {
        ParseInstruction::Tag(name, order) => {
            let nodes: Find<'_, Name<&str>> = root_node.find(Name(name.as_str()));
            let instruction = ParseInstruction::Tag("a".to_string(), order.clone());
            nodes
                .map(|n| get_node_href(&instruction, n, html))
                .collect()
        }
        ParseInstruction::Class(name, order) => {
            let nodes: Find<'_, Class<&str>> = root_node.find(Class(name.as_str()));
            let instruction = ParseInstruction::Tag("a".to_string(), order.clone());
            nodes
                .into_iter()
                .map(|n| get_node_href(&instruction, n, html))
                .collect()
        }
    };

    return Ok(a);
}
