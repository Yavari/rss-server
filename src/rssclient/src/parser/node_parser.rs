use select::{
    node::Node,
    predicate::{Class, Name, Predicate},
};

use super::{Order, ParseInstruction};

pub fn get_node<'a>(
    instruction: &ParseInstruction,
    node: Node<'a>,
    html: &str,
) -> Result<Node<'a>, String> {
    match instruction {
        ParseInstruction::Tag(name, order) => {
            f(node.find(Name(name.as_str())), &name, order, &html[0..100])
        }
        ParseInstruction::Class(name, order) => {
            f(node.find(Class(name.as_str())), &name, order, &html[0..100])
        }
    }
}

pub fn get_node_as_text(
    instruction: &ParseInstruction,
    article: Node,
    html: &str,
) -> Result<String, String> {
    let node = get_node(instruction, article, html)?;
    Ok(node.text())
}

pub fn get_node_href(
    instruction: &ParseInstruction,
    node: Node,
    html: &str,
) -> Result<String, String> {
    let node = get_node(instruction, node, html)?;
    let a = node
        .attr("href")
        .ok_or("Cound not parse href".to_string())
        .and_then(|url| Ok(url.to_string()));
    a
}
pub fn f<'a, P: Predicate>(
    node: select::node::Find<'a, P>,
    name: &str,
    order: &Order,
    scope: &str,
) -> Result<Node<'a>, String> {
    match order {
        Order::Normal(n) => node.skip(*n).next().ok_or(format!(
            "Could not find {} at element {} in {}",
            name, n, scope
        )),
        Order::Reverse(n) => {
            let node: Vec<_> = node.collect();
            node.into_iter().rev().skip(*n).next().ok_or(format!(
                "Could not find {} at element {} in {}",
                name, n, scope
            ))
        }
    }
}
