#![recursion_limit="256"]

///http://docutils.sourceforge.net/docs/ref/doctree.html
///serves as AST

#[macro_use]
mod macro_util;

pub mod url;
pub mod elements;
pub mod element_categories;
pub mod extra_attributes;
pub mod attribute_types;

pub use self::elements::*; //Element,CommonAttributes,HasExtraAndChildren
pub use self::extra_attributes::ExtraAttributes;
pub use self::element_categories::HasChildren;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn imperative() {
		let mut doc = Document::default();
		let mut title = Title::default();
		title.append_child("Hi");
		doc.append_child(title);

		println!("{:?}", doc);
	}

	#[test]
	fn descriptive() {
		let doc = Document::with_children(vec![
			Title::with_children(vec![
				"Hi".into()
			]).into()
		]);

		println!("{:?}", doc);
	}
}
