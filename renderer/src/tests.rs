use document_tree::Document;

use crate::render_xml;

const XML_DECL: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

#[test]
fn test_render_xml() {
    let mut s = Vec::new();
    // should probably be lowercase
    let exp = "<Document />";

    render_xml(&Document::default(), &mut s, true).unwrap();
    assert_eq!(std::str::from_utf8(&s).unwrap(), format!("{XML_DECL}{exp}"));

    render_xml(&Document::default(), &mut s, false).unwrap();
    assert_eq!(std::str::from_utf8(&s).unwrap(), exp);
}
