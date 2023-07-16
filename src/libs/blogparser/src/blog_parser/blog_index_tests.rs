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
        let parsed = get_ok(&links);
        assert_eq!(parsed, vec!["url1", "url2"]);

        let error = get_err(&links);
        assert_eq!(error, vec!["Could not find ordered element Selector { selectors: [Selector(a, specificity = 0x1, flags = SelectorFlags(0x0))] } Normal(0)"]);
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
        let parsed = get_ok(&links);
        assert_eq!(parsed, vec!["url1", "url2"]);

        let error = get_err(&links);
        assert_eq!(error.len(), 0);
    }

    fn get_ok(links: &Vec<Result<String, crate::BlogError>>) -> Vec<String> {
        links.into_iter().filter_map(|x| x.as_ref().ok().cloned()).collect()
    }

    fn get_err(links: &Vec<Result<String, crate::BlogError>>) -> Vec<String> {
        links
            .into_iter()
            .filter_map(|x| x.as_ref().err())
            .map(|x| x.to_string())
            .collect()
    }
}
