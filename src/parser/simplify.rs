use std::collections::HashMap;

use crate::target::Target;
use crate::document_tree::{
	Document,
	HasChildren,
	attribute_types::ID,
	element_categories as c,
};


trait ResolvableRefs {
	fn populate_targets(&self, refs: &mut HashMap<&ID, Target>);
	fn resolve_refs(self, refs: &HashMap<&ID, Target>) -> Self;
}

pub fn resolve_references(mut doc: Document) -> Document {
	let mut references = HashMap::new();
	for c in doc.children() {
		c.populate_targets(&mut references);
	}
	let new: Vec<_> = doc.children_mut().drain(..).map(|c| c.resolve_refs(&references)).collect();
	Document::with_children(new)
}

impl ResolvableRefs for c::StructuralSubElement {
	fn populate_targets(&self, refs: &mut HashMap<&ID, Target>) {
		//TODO
	}
	fn resolve_refs(self, refs: &HashMap<&ID, Target>) -> Self {
		self //TODO
	}
}

