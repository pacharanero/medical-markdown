use wasm_bindgen::prelude::*;

fn build_parser() -> markdown_it::MarkdownIt {
    let mut md = markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(&mut md);
    medical_markdown::add(&mut md);
    md
}

/// Parse medical markdown and return rendered HTML.
#[wasm_bindgen]
pub fn parse_to_html(input: &str) -> String {
    let md = build_parser();
    let ast = md.parse(input);
    ast.render()
}

/// Parse medical markdown and return structured JSON.
#[wasm_bindgen]
pub fn parse_to_json(input: &str) -> String {
    let md = build_parser();
    let ast = md.parse(input);
    let data = medical_markdown::extract_structured_data(&ast);
    serde_json::to_string_pretty(&data).unwrap_or_default()
}

/// Parse medical markdown and return both HTML and structured JSON
/// as a JSON object: `{ "html": "...", "structured": {...} }`.
#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    let md = build_parser();
    let ast = md.parse(input);
    let html = ast.render();
    let structured = medical_markdown::extract_structured_data(&ast);

    let mut result = serde_json::Map::new();
    result.insert("html".to_string(), serde_json::Value::String(html));
    result.insert("structured".to_string(), structured);
    serde_json::to_string(&serde_json::Value::Object(result)).unwrap_or_default()
}
