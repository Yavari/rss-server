use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

static SELECT_ALL: Lazy<Selector> = Lazy::new(|| Selector::parse("*").unwrap());

pub struct RegexParser {
    document: Html,
}

impl RegexParser {
    pub fn create(html: &str, re: &String) -> RegexParser {
        let re = Regex::new(re).unwrap();
        RegexParser {
            document: Html::parse_fragment(get_single_element(html, re).trim()),
        }
    }

    pub fn create_vec(html: &str, re: &String) -> Vec<RegexParser> {
        let re = Regex::new(re).unwrap();
        get_html_from_regex(html, re)
            .into_iter()
            .map(|element| RegexParser {
                document: Html::parse_fragment(&element),
            })
            .collect()
    }

    pub fn get_element_ref<'a>(&'a self) -> ElementRef<'a> {
        self.document.select(&SELECT_ALL).next().unwrap()
    }
}

fn get_single_element(html: &str, re: Regex) -> String {
    let (_, [path]) = re
        .captures_iter(html)
        .next()
        .expect(&format!("Could not find matches for {}", re))
        .extract();
    return path.to_string();
}

fn get_html_from_regex(html: &str, re: Regex) -> Vec<String> {
    re.captures_iter(html)
        .map(|c| {
            let (_, [path]) = c.extract();
            path.to_string()
        })
        .collect()
}
