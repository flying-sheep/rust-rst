`document_tree`
===============

Part of the [`rst`][rst] crate family.
This crate contains structs and traits mirroring [Docutils’ Document Tree][doctree] model.
The basic structure is a tree of [elements][], some of which [have children][] and/or [extra attributes][].

```rust
use document_tree::*;
use document_tree::{extra_attributes as a, element_categories as c, attribute_types as t};

#[test]
fn imperative() {
    let mut doc = Document::default();
    let mut title = Title::default();
    let url = "https://example.com/image.jpg".parse().unwrap();
    let image = ImageInline::with_extra(a::ImageInline::new(url));
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
            ImageInline::with_extra(a::ImageInline::new(
                "https://example.com/image.jpg".parse().unwrap()
            )).into(),
        ]).into()
    ]);
    println!("{:?}", doc);
}
```

Check out the other crates in the family on how to create one from rST markup or render it!

The advantages of this approach are that it’s convenient to have the children interface,
as well as to trivially map elements to XML.
The disadvantage is that a “vector of children” is not a well-defined model for the more structured elements
like e.g. a section, which always contains a title followed by blocks.

[rst]: https://github.com/flying-sheep/rust-rst/#readme
[doctree]: https://docutils.sourceforge.io/docs/ref/doctree.html
[elements]: https://docs.rs/document_tree/0/document_tree/elements/trait.Element.html
[have children]: https://docs.rs/document_tree/0/document_tree/element_categories/trait.HasChildren.html
[extra attributes]: https://docs.rs/document_tree/0/document_tree/extra_attributes/trait.ExtraAttributes.html
