/*
http://docutils.sourceforge.net/docs/ref/rst/restructuredtext.html#hyperlink-targets

Links can have internal or external targets.
In the source, targets look like:

	.. targetname1:
	.. targetname2:

	some paragraph or list item or so

or:

    .. targetname1:
	.. targetname2: https://link

There’s also anonymous links and targets without names.

TODO: continue documenting how it’s done via http://svn.code.sf.net/p/docutils/code/trunk/docutils/docutils/transforms/references.py
*/

use std::collections::HashMap;

use crate::target::Target;
use crate::document_tree::{
	Document,
	HasChildren,
	attribute_types::ID,
	elements as e,
	element_categories as c,
};


enum MaybeDirectTarget {
	IndirectTarget(ID),
	DirectTarget(Target),
}

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

fn sub_pop<P, C>(parent: &P, refs: &mut HashMap<&ID, Target>) where P: HasChildren<C>, C: ResolvableRefs {
	for c in parent.children() {
		c.populate_targets(&mut refs);
	}
}

fn sub_res<P, C>(parent: P, refs: &HashMap<&ID, Target>) -> P where P: e::Element + HasChildren<C>, C: ResolvableRefs {
	
	let new: Vec<_> = parent.children_mut().drain(..).map(|c| c.resolve_refs(&refs)).collect();
	parent.children_mut().extend(new);
	parent
}

impl ResolvableRefs for c::StructuralSubElement {
	fn populate_targets(&self, refs: &mut HashMap<&ID, Target>) {
		use c::StructuralSubElement::*;
		match *self {
			Title(e)        => sub_pop(&*e, refs),
			Subtitle(e)     => sub_pop(&*e, refs),
			Decoration(e)   => sub_pop(&*e, refs),
			Docinfo(e)      => sub_pop(&*e, refs),
			SubStructure(e) => e.populate_targets(refs),
		}
	}
	fn resolve_refs(self, refs: &HashMap<&ID, Target>) -> Self {
		use c::StructuralSubElement::*;
		match self {
			Title(e)        => sub_res(*e, refs).into(),
			Subtitle(e)     => sub_res(*e, refs).into(),
			Decoration(e)   => sub_res(*e, refs).into(),
			Docinfo(e)      => sub_res(*e, refs).into(),
			SubStructure(e) => e.resolve_refs(refs).into(),
		}
	}
}

impl ResolvableRefs for c::SubStructure {
	fn populate_targets(&self, refs: &mut HashMap<&ID, Target>) {
		use c::SubStructure::*;
		match *self {
			Topic(e) => sub_pop(&*e, refs),
			Sidebar(e) => sub_pop(&*e, refs),
			Transition(e) => sub_pop(&*e, refs),
			Section(e) => sub_pop(&*e, refs),
			BodyElement(e) => e.populate_targets(refs),
		}
	}
	fn resolve_refs(self, refs: &HashMap<&ID, Target>) -> Self {
		use c::SubStructure::*;
		match self {
			Topic(e) => sub_res(*e, refs).into(),
			Sidebar(e) => sub_res(*e, refs).into(),
			Transition(e) => sub_res(*e, refs).into(),
			Section(e) => sub_res(*e, refs).into(),
			BodyElement(e) => e.resolve_refs(refs).into(),
		}
	}
}
