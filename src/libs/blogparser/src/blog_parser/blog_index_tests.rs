#[cfg(test)]
mod tests {
    use crate::{blog_parser::parse_links, BlogIndex, Order, ParseInstruction};
    #[test]
    fn can_get_links_from_li_list() {
        let blog = BlogIndex {
            section: ParseInstruction::Selectors(".col-md-10".to_string(), Order::Normal(0)),
            link_selector: ParseInstruction::Selectors("li".to_string(), Order::Normal(0)),
        };
        let html = r#"<div class="col-md-10"><ul>
        <li><a href="url1">Title1</a></li>
        <li><a href="url2">Title2</a></li>
        <li></li>
        </ul></div>"#.to_string();
        let links = parse_links(&blog, &html).unwrap();
        assert_eq!(links, vec!["url1", "url2"]);
    }

    #[test]
    fn can_get_links_from_regex() {
        let blog = BlogIndex {
            section: ParseInstruction::Selectors(".col-md-10".to_string(), Order::Normal(0)),
            link_selector: ParseInstruction::Regex(r#"<a href="(.*)">"#.to_string()),
        };
        let html = r#"<div class="col-md-10"><ul>
        <li><a href="url1">Title1</a></li>
        <li><a href="url2">Title2</a></li>
        <li></li>
        </ul></div>"#.to_string();
        let links = parse_links(&blog, &html).unwrap();
        assert_eq!(links, vec!["url1", "url2"]);
    }
}
