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
	use std::default::Default;

	#[test]
	fn imperative() {
		let mut doc = Document::default();
		let mut title = Title::default();
		let url = "https://example.com/image.jpg".parse().unwrap();
		let image = ImageInline::with_extra(extra_attributes::ImageInline::new(url));
		title.append_child("Hi");
		title.append_child(image);
		doc.append_child(title);

		println!("{:?}", doc);
	}

	#[test]
	fn descriptive() {
		let doc = Document::with_children(vec![
			Title::with_children(vec![
				"Hi".into(),
				ImageInline::with_extra(extra_attributes::ImageInline::new(
					"https://example.com/image.jpg".parse().unwrap()
				)).into(),
			]).into()
		]);

		println!("{:?}", doc);
	}
}
