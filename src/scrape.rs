use scraper::{Html, Selector};

pub fn extract_fbzx(html: &str) -> Option<String> {
    let document = Html::parse_document(html);

    let selector = Selector::parse(r#"input[name="fbzx"]"#).ok()?;

    let element = document.select(&selector).next()?;

    element.value().attr("value").map(|v| v.to_string())
}
