use ammonia::clean;
use pulldown_cmark::{Parser, html};
use yew::{AttrValue, Html};

pub fn markdown_to_html_safe(markdown_input: &str) -> Html {
    println!("Markdown input: {}", markdown_input);
    let parser = Parser::new(markdown_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let sanitized = clean(&html_output);
    Html::from_html_unchecked(AttrValue::from(sanitized))
}
