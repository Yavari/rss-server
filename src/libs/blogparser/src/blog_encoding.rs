use crate::{Blog, BlogError};

impl Blog {
    pub fn to_html_safe_string(&self) -> String {
        let json = serde_json::to_string(self).unwrap();
        let query = &[("json", json)];
        serde_urlencoded::to_string(query).unwrap()
    }

    pub fn from_html_safe_string(query: &str) -> Blog {
        let b = serde_urlencoded::from_str::<Vec<(String, String)>>(query).unwrap();
        let (_, b) = b.into_iter().next().unwrap();
        serde_json::from_str(&b).unwrap()
    }

    pub fn from_json(json: &str) -> Result<Blog, BlogError> {
        serde_json::from_str(&json).map_err(|x| BlogError::FromJsonParseError(x, json.to_owned()))
    }
}
