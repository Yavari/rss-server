use axum::{extract::State, response::Html, Json};
use blogparser::{parse_links, Blog};

use crate::AppState;

pub async fn view_blog(state: State<AppState>, Json(blog): Json<Blog>) -> Html<String> {
    println!("hej");
    let response = blog.fetch_blog(&state.client).await;
    if let Ok(response) = response {
        let urls = parse_links(&blog.index, &response);
        let urls = urls;
        if let Ok(urls) = urls {
            let a = urls
                .iter()
                .map(|f| format!("<a href='/rss/blogs/articles{}'>{}</a>", f, f))
                .collect::<Vec<String>>()
                .join("<br/>");

            return Html(a);
        }
    }
    Html("Error".to_string())
}
