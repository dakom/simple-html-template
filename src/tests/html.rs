use crate::*;

#[test]
fn whitespace() {
    let t = Template::new(" <div>hello world</div> ").unwrap();
    let s = t.render_plain();

    assert_eq!(s, "<div>hello world</div>");
}

#[test]
fn html_string() {
    let args = html_map! {
        0 => "Cat",
        1 => "<b>Cat</b>",
    };

    
    assert_eq!(*args.get(&0).unwrap(), "Cat");
    assert_eq!(*args.get(&1).unwrap(), "&lt;b&gt;Cat&lt;/b&gt;");
}

#[test]
fn html_template() {

    let template = Template::new("${capitalized specific} are my favorite ${category}.").unwrap();

    let args = html_map!{
        "capitalized specific" => "<b>Cats</b>",
        "category" => "<i>animal</i>"
    };

    let s = template.render(&args).unwrap();

    assert_eq!(s, "&lt;b&gt;Cats&lt;/b&gt; are my favorite &lt;i&gt;animal&lt;/i&gt;.");

}
