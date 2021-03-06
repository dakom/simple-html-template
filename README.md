# Simple Html Template 

[![Crates.io](https://img.shields.io/crates/v/simple-html-template)](https://crates.io/crates/simple-html-template)
[![Documentation](https://docs.rs/simple-html-template/badge.svg)](https://docs.rs/simple-html-template)

This is essentially a fork of [far](https://crates.io/crates/far), with some additions:
  * a cache for re-use, as well as a higher-level cache for lookup by name
  * macro to make safe html variables.
  * (optional, disabled by default) helpers to work with the DOM in a wasm context 

The `html_map!` and `html_map_strong!` macros use the [htmlescape](https://crates.io/crates/htmlescape) crate, which you must add as a dependency (or else compilation will fail when calling these macros)

Note that, like `far`, this crate does not deal with escaping the keys or replacements in any way. e.g. if for some reason you need the template to have a `${}` literal.

The value of the HashMap which is passed to Template::render() must be `AsRef<str>`

Examples:

---

Provided with a string and a map, simple-html-template will attempt to find
all the keys (delimited with `${}`) in the template and replace them with
the corresponding value in the map. For example:

```rust
let template = Template::new("${capitalized specific} are my favorite ${category}.")?;

let args = html_map!{
    "capitalized specific" => "Cats",
    "category" => "animal",
};

let s = template.render(&args)?;

assert_eq!(s, "Cats are my favorite animal.");
```

If it fails for some reason, an explanation of why will be returned:

```rust
let template = Template::new("${capitalized specific} are my favorite ${category}.")?;

let args = html_map!{
    "capitalized specific" => "Cats",
    // Note the typo here
    "catglory" => "animal",
};


match template.render(&args) {
    Ok(_) => panic!(),
    Err(e) => {
        assert_eq!(
            format!("{}", e),
            r#"missing key: "category"; extraneous key: "catglory""#
        );
    }
}
```

Note that if html is in the variable, it is escaped:

```rust
let template = Template::new("${capitalized specific} are my favorite ${category}.")?;

let args = html_map!{
    "capitalized specific" => "<b>Cats</b>",
    "category" => "<i>animal</i>",
};

let s = template.render(&args)?;

assert_eq!(s, "&lt;b&gt;Cats&lt;/b&gt; are my favorite &lt;i&gt;animal&lt;/i&gt;.");
```
Additional examples and weird edge-case behaviors can be found in
`src/tests`.

## License

This project is licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)

* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)
at your option.
