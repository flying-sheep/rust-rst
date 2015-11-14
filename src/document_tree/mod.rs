///http://docutils.sourceforge.net/docs/ref/doctree.html
///serves as AST

pub mod elements;
pub mod element_categories;
pub mod extra_attributes;
pub mod attribute_types;

pub use self::elements::*; //Element,CommonAttributes,
pub use self::extra_attributes::ExtraAttributes;
pub use self::element_categories::HasChildren;

#[test]
fn test() {
	use document_tree as dt;
	use document_tree::HasChildren;
	
	let mut doc = dt::Document::default();
	let mut title = dt::Title::default();
	title.append_child("Hi");
	doc.append_child(title);
	
	println!("{:?}", doc);
}
