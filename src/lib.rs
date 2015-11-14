extern crate url;

pub mod document_tree;

#[test]
fn test() {
	use document_tree as dt;
	use document_tree::HasChildren;
	
	let mut doc = dt::Document::default();
	let title = dt::Title::default();
	doc.append_child(title);
	
	println!("{:?}", doc);
}
