use pretty_assertions::assert_eq;

use crate::parser::parse;
use super::html::render_html;

fn check_renders_to(rst: &str, expected: &str) {
	println!("Rendering:\n{}\n---", rst);
	let doc = parse(rst).expect("Cannot parse");
	let mut result_data: Vec<u8> = vec![];
	render_html(&doc, &mut result_data, false).expect("Render error");
	let result = String::from_utf8(result_data).expect("Could not decode");
	assert_eq!(result.as_str().trim(), expected);
}

include!(concat!(env!("OUT_DIR"), "/html_tests.rs"));
