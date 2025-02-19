#![recursion_limit = "256"]

//! See [doctree][] reference.
//! Serves as AST.
//!
//! [doctree]: http://docutils.sourceforge.net/docs/ref/doctree.html

#[macro_use]
mod macro_util;

pub mod attribute_types;
pub mod element_categories;
pub mod elements;
pub mod extra_attributes;
pub mod url;

pub use self::element_categories::HasChildren;
pub use self::elements::*; //Element,CommonAttributes,HasExtraAndChildren
pub use self::extra_attributes::ExtraAttributes;

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
        let doc = Document::with_children(vec![Title::with_children(vec![
            "Hi".into(),
            ImageInline::with_extra(extra_attributes::ImageInline::new(
                "https://example.com/image.jpg".parse().unwrap(),
            ))
            .into(),
        ])
        .into()]);

        println!("{:?}", doc);
    }
}
