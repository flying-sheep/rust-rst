///http://docutils.sourceforge.net/docs/ref/doctree.html
///serves as AST

pub mod elements;
pub mod element_categories;
pub mod extra_attributes;
pub mod attribute_types;

use self::element_categories::StructuralSubElement;

pub use self::elements::*; //Element,CommonAttributes,
pub use self::extra_attributes::ExtraAttributes;
pub use self::element_categories::HasChildren;

#[derive(Default,Debug)]
pub struct Document { children: Vec<Box<StructuralSubElement>> }
impl HasChildren<StructuralSubElement> for Document {
	fn add_child<R: Into<StructuralSubElement>>(&mut self, child: R) {
		self.children.push(Box::new(child.into()));
	}
}
