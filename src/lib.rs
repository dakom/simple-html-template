use std::collections::HashMap;
mod errors;
#[cfg(test)]
mod tests;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm")]
use wasm_bindgen::JsCast;
#[cfg(feature = "wasm")]
use web_sys::{Document, DocumentFragment, HtmlTemplateElement, HtmlElement};

pub use errors::{Error, Errors};

#[macro_export]
macro_rules! hash_map(
    { $($key:expr => $value:expr),* $(,)? } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

#[macro_export]
macro_rules! html_map(
    { $($key:expr => $value:expr),* $(,)? } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, htmlescape::encode_minimal($value));
            )+
            m
        }
     };
);

#[macro_export]
macro_rules! html_map_strong(
    { $($key:expr => $value:expr),* $(,)? } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, htmlescape::encode_attribute($value));
            )+
            m
        }
     };
);

pub struct Template<'a> {
    // Stores (key, (key_start, key_end))
    pub replaces: Vec<(&'a str, (usize, usize))>,
    pub template_str:&'a str,
}


impl <'a> Template <'a> {
    pub fn new (template_str: &'a str) -> Result<Self, Error> {
        let mut template = Self { replaces: Vec::new(), template_str };

        let replaces = &mut template.replaces;

        // Current position in the format string
        let mut cursor = 0;

        while cursor < template_str.len() {
            if let Some(start) = (&template_str[cursor..]).find("${") {
                let start = start + cursor;
                if let Some(end) = (&template_str[cursor..]).find('}') {
                    let end = end + cursor;
                    replaces.push((
                        // The extracted key
                        &template_str[(start + "${".len())..end],
                        (
                            // Points to the `$` in the `${`
                            start,
                            // Just after the matching `}`
                            (end + "}".len()),
                        ),
                    ));

                    // Move cursor to the end of this match
                    cursor = end + "}".len();
                } else {
                    // Bail immediately: if there's an unclosed delimiter, then
                    // we basically can't guess about what provided key-value
                    // pairs are needed
                    return Err(Error::Unclosed(start));
                }
            } else {
                // No more matches
                break;
            }
        }
        Ok(template)
    }

    pub fn render<V: AsRef<str>>(&self, vars:&HashMap<&str, V>) -> Result<String, Errors> {
        let mut errors = Vec::new();
        let replaces = &self.replaces;
        let template_str = &self.template_str;

        for k in vars.keys() {
            if !replaces.iter().any(|(x, (_, _))| x == k) {
                errors.push(Error::Extra((*k).to_string()));
            }
        }

        // Wait on bailing out if there are errors so we can display all the errors
        // at once instead of making the user have to try to fix it twice.

        // Calculate the size of the text to be added (vs) and the amount of space
        // the keys take up in the original text (ks)
        let (ks, vs) = replaces.iter().fold((0, 0), |(ka, va), (k, _)| {
            if let Some(v) = vars.get(k) {
                (ka + k.len(), va + v.as_ref().len())
            } else {
                errors.push(Error::Missing((*k).to_string()));

                // This is mostly just to get past the typechecker
                (ka, va)
            }
        });

        // If there were errors, bail out
        if !errors.is_empty() {
            return Err(Errors {
                inner: errors,
            });
        }

        let final_len = (template_str.len() - ("${}".len() * replaces.len())) + vs - ks;

        let mut output = String::with_capacity(final_len);

        let mut cursor:usize = 0;

        for (key, (start, end)) in replaces.into_iter() {
            output.push_str(&template_str[cursor..*start]);
            // Unwrapping should be safe at this point because we should have caught
            // it while calculating replace_size.
            output.push_str(vars.get(key).unwrap().as_ref());
            cursor = *end;
        }

        // If there's more text after the final `${}`
        if cursor < template_str.len() {
            output.push_str(&template_str[cursor..]);
        }

        #[cfg(test)]
        assert_eq!(output.len(), final_len);

        Ok(output)
    }

    #[cfg(feature = "wasm")]
    pub fn render_fragment<V: AsRef<str>>(&self, doc:&Document, data:&HashMap<&str, V>) -> Result<DocumentFragment, Errors> {
        let html = self.render(data)?;
        let el: HtmlTemplateElement = doc.create_element("template").unwrap_throw().unchecked_into();
        el.set_inner_html(&html);
        Ok(el.content())
    }

    #[cfg(feature = "wasm")]
    pub fn render_fragment_plain(&self, doc:&Document) -> DocumentFragment {
        let el: HtmlTemplateElement = doc.create_element("template").unwrap_throw().unchecked_into();
        el.set_inner_html(&self.template_str);
        el.content()
    }

    #[cfg(feature = "wasm")]
    pub fn render_elem<V: AsRef<str>>(&self, doc:&Document, data:&HashMap<&str, V>) -> Result<HtmlElement, Errors> {
        self.render_fragment(doc, data)
            .map(|frag| {
                frag.first_child().unwrap().unchecked_into()
            })
    }

    #[cfg(feature = "wasm")]
    pub fn render_elem_plain(&self, doc:&Document) -> HtmlElement {
        let frag = self.render_fragment_plain(doc);
        frag.first_child().unwrap_throw().unchecked_into()
    }
}


/// render functions panic if the template name doesn't exist
pub struct TemplateCache <'a> {
    pub templates: HashMap<&'a str, Template<'a>>,
    #[cfg(feature = "wasm")]
    pub doc: Document,
}

impl <'a> TemplateCache <'a> {

    pub fn new(templates:&[(&'a str, &'a str)]) -> Self{
        let mut _templates = HashMap::new();

        for (name, data) in templates {
            _templates.insert(*name, Template::new(data).unwrap());
        }

        Self::_new(_templates)
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "wasm")] {
            fn _new(_templates:HashMap<&'a str, Template<'a>>) -> Self {
                let window = web_sys::window().unwrap_throw();
                let doc = window.document().unwrap_throw();

                Self { templates: _templates, doc }
            }
        } else {
            fn _new(_templates:HashMap<&'a str, Template<'a>>) -> Self {
                Self {templates: _templates }
            }
        }
    }

    pub fn render<V: AsRef<str>>(&self, name:&str, data:&HashMap<&str,V>) -> Result<String, Errors> {
        self.templates.get(name).unwrap().render(data)
    }

    pub fn render_plain(&self, name:&str) -> &str {
        self.templates.get(name).unwrap().template_str
    }

    #[cfg(feature = "wasm")]
    pub fn render_fragment<V: AsRef<str>>(&self, name:&str, data:&HashMap<&str, V>) -> Result<DocumentFragment, Errors> {
        self.templates.get(name).unwrap_throw().render_fragment(&self.doc, data)
    }

    #[cfg(feature = "wasm")]
    pub fn render_fragment_plain(&self, name:&str) -> DocumentFragment {
        self.templates.get(name).unwrap_throw().render_fragment_plain(&self.doc)
    }

    #[cfg(feature = "wasm")]
    pub fn render_elem<V: AsRef<str>>(&self, name:&str, data:&HashMap<&str, V>) -> Result<HtmlElement, Errors> {
        self.templates.get(name).unwrap_throw().render_elem(&self.doc, data)
    }

    #[cfg(feature = "wasm")]
    pub fn render_elem_plain(&self, name:&str) -> HtmlElement {
        self.templates.get(name).unwrap_throw().render_elem_plain(&self.doc)
    }
}
