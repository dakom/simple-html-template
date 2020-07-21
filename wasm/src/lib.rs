use wasm_bindgen::prelude::*;
use simple_html_template::Template;
use std::collections::HashMap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// render the tempate while html-escaping each value of the lookup
/// unwraps errors
#[wasm_bindgen]
pub fn render_template(template_str:String, lookup:JsValue) -> String {
    let mut lookup:HashMap<String, String> = serde_wasm_bindgen::from_value(lookup).unwrap();

    for (_, val) in lookup.iter_mut() {
        *val = htmlescape::encode_minimal(val);
    }

    _render_template(template_str, lookup)
}

/// render the tempate while strong html-escaping each value of the lookup (for attributes)
/// unwraps errors
#[wasm_bindgen]
pub fn render_template_strong(template_str:String, lookup:JsValue) -> String {
    let mut lookup:HashMap<String,String> = serde_wasm_bindgen::from_value(lookup).unwrap();

    for (_, val) in lookup.iter_mut() {
        *val = htmlescape::encode_attribute(val);
    }

    _render_template(template_str, lookup)
}

/// render the tempate with no html-escaping each value of the lookup
/// unwraps errors
#[wasm_bindgen]
pub fn render_template_unsafe(template_str:String, lookup:JsValue) -> String {
    let mut lookup:HashMap<String,String> = serde_wasm_bindgen::from_value(lookup).unwrap();
    _render_template(template_str, lookup)
}

fn _render_template(template_str:String, lookup:HashMap<String, String>) -> String {
    let mut lookup_ref:HashMap<&str, &str> = HashMap::new();
    lookup.iter().for_each(|(k, v)| {
        lookup_ref.insert(k, v);
    });

    let template = Template::new(&template_str).unwrap();
    template.render(&lookup_ref).unwrap()
}
