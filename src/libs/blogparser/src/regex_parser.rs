use regex::Regex;

pub struct RegexParser {
    html: String,
}

impl RegexParser {
    pub fn create(html: &str, re: &String) -> RegexParser {
        let re = Regex::new(re).unwrap();
        RegexParser {
            html: get_single_element(html, re).trim().to_string(),
        }
    }

    pub fn create_vec(html: &str, re: &String) -> Vec<RegexParser> {
        let re = Regex::new(re).unwrap();
        get_html_from_regex(html, re)
            .into_iter()
            .map(|html| RegexParser { html: html })
            .collect()
    }

    pub fn html(&self) -> &str {
        &self.html
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
