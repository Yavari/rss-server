use crate::{Blog, BlogError};

impl Blog {
    pub fn to_html_safe_string(&self) -> Result<String, BlogError> {
        let json = serde_json::to_string(self)
            .map_err(|x| BlogError::Generic(format!("Could not serialize to string {}", x)))?;
        let query = &[("json", json)];
        serde_urlencoded::to_string(query).map_err(|x| BlogError::Generic(format!("Could not serde_urlencoded::to_string {}", x)))
    }

    pub fn from_html_safe_string(query: &str) -> Result<Blog, BlogError> {
        let b = serde_urlencoded::from_str::<Vec<(String, String)>>(query)
            .map_err(|x| BlogError::Generic(format!("Could not serde_urlencoded::from_str {}", x)))?;
        let (_, b) = b
            .into_iter()
            .next()
            .ok_or(BlogError::Generic("No elements in json".to_string()))?;
        Blog::from_json(b)
    }

    pub fn from_json(json: String) -> Result<Blog, BlogError> {
        serde_json::from_str(&json).map_err(|x| BlogError::FromJsonParseError(x, json))
    }
}
