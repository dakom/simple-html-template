# String Template 

[![version](https://img.shields.io/crates/v/simple-html-template?label=version)](https://crates.io/crates/simple-html-template)
[![license](https://img.shields.io/crates/l/simple-html-template?label=license)](https://crates.io/crates/simple-html-template)
[![downloads](https://img.shields.io/crates/d/simple-html-template?label=downloads)](https://crates.io/crates/simple-html-template)
[![chat](https://img.shields.io/matrix/users:typ3.tech)](https://matrix.to/#/#users:typ3.tech)

This is essentially a fork of [far](https://crates.io/crates/far), with a cache for re-use and an optional macro (enabled by default) to make safe html variables.

The `html_map!` macro uses the minimal set of escapes (as per [The XSS cheatsheet](https://cheatsheetseries.owasp.org/cheatsheets/Cross_Site_Scripting_Prevention_Cheat_Sheet.html)). If you need to have stronger guarantees, i.e. for sending data for attributes, then use `html_map_strong!` or build the hashmap the usual way and wrap the values with htmlescape::encode_attribute().

Note that, like `far`, this crate does not deal with escaping the keys or replacements in any way. e.g. if for some reason you need the template to have a `${}` literal.

The value of the HashMap which is passed to Template::render() must be `AsRef<str>`

The only dependency is the optional `htmlescape`

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
