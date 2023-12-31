use regex::Regex;

use crate::BlogError;

pub fn regex_parse(html: &str, re: &str) -> Result<Vec<String>, BlogError> {
    let re = Regex::new(re).map_err(BlogError::RegEx)?;
    let result = re
        .captures_iter(html)
        .map(|c| {
            let (_, [path]) = c.extract();
            path.trim().to_string()
        })
        .collect();
    Ok(result)
}

pub fn regex_parse_single_node(html: &str, re: &str) -> Result<String, BlogError> {
    let list = regex_parse(html, re)?;
    let item = list
        .first()
        .ok_or(BlogError::Generic("Could not find node".to_string()))?;
    Ok(item.to_string())
}
