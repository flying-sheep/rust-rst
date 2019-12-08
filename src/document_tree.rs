///http://docutils.sourceforge.net/docs/ref/doctree.html
///serves as AST

#[macro_use]
mod macro_util;

pub mod elements;
pub mod element_categories;
pub mod extra_attributes;
pub mod attribute_types;

pub use self::elements::*; //Element,CommonAttributes,HasExtraAndChildren
pub use self::extra_attributes::ExtraAttributes;
pub use self::element_categories::HasChildren;

#[test]
fn test_imperative() {
	let mut doc = Document::default();
	let mut title = Title::default();
	title.append_child("Hi");
	doc.append_child(title);
	
	println!("{:?}", doc);
}

#[test]
fn test_descriptive() {
	let doc = Document::with_children(vec![
		Title::with_children(vec![
			"Hi".into()
		]).into()
	]);
	
	println!("{:?}", doc);
}
