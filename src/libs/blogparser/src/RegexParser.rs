use regex::Regex;
use scraper::{Html, ElementRef, Selector};

pub struct RegexParser {
    document: Html,
}

impl RegexParser {
    pub fn create(content_node: ElementRef<'_>, x: &String) -> RegexParser {
        let html = get_html_from_regex(content_node, x).unwrap();
        RegexParser {
            document: Html::parse_fragment(html.trim()),
        }
    }
    pub fn get_element_ref<'a>(&self) -> ElementRef<'_> {
        self.document.select(&Selector::parse("*").unwrap()).next().unwrap()
    }
}

fn get_html_from_regex(content_node: ElementRef<'_>, x: &String) -> Option<String> {
    let html = content_node.html();
    let re = Regex::new(x).unwrap();
    for (_, [path]) in re.captures_iter(&html).map(|c| c.extract()) {
        return Some(path.to_string());
    }

    None
}