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
	attribute_types::{ID, NameToken},
	elements as e,
	element_categories as c,
};


#[derive(Debug)]
enum NamedTargetType {
	NumberedFootnote(usize),
	LabeledFootnote(usize),
	Citation,
	InternalLink,
	ExternalLink(Target),
	IndirectLink(NameToken),
	SectionTitle
}
impl NamedTargetType {
	fn is_implicit_target(&self) -> bool {
		match self {
			NamedTargetType::SectionTitle => true,
			_ => false
		}
	}
}

#[derive(Clone, Debug)]
struct Substitution {
	content: e::ImageInline,
	// If `ltrim` is true and the sibling before the reference is a text node,
	// the text node gets right-trimmed. Same for `rtrim` with the sibling after
	// getting left-trimmed.
	ltrim: bool,
	rtrim: bool
}

#[derive(Default, Debug)]
struct TargetsCollected {
	named_targets: HashMap<NameToken, NamedTargetType>,
	substitutions: HashMap<String, Substitution>,
	normalized_substitutions: HashMap<String, Substitution>
}

trait ResolvableRefs {
	fn populate_targets(&self, refs: &mut TargetsCollected);
	fn resolve_refs(self, refs: &TargetsCollected) -> Self;
}

pub fn resolve_references(mut doc: Document) -> Document {
	let mut references: TargetsCollected = Default::default();
	for c in doc.children() {
		c.populate_targets(&mut references);
	}
	let new: Vec<_> = doc.children_mut().drain(..).map(|c| c.resolve_refs(&references)).collect();
	Document::with_children(new)
}

fn sub_pop<P, C>(parent: &P, refs: &mut TargetsCollected) where P: HasChildren<C>, C: ResolvableRefs {
	for c in parent.children() {
		c.populate_targets(refs);
	}
}

fn sub_res<P, C>(mut parent: P, refs: &TargetsCollected) -> P where P: e::Element + HasChildren<C>, C: ResolvableRefs {
	let new: Vec<_> = parent.children_mut().drain(..).map(|c| c.resolve_refs(refs)).collect();
	parent.children_mut().extend(new);
	parent
}

fn sub_sub_pop<P, C1, C2>(parent: &P, refs: &mut TargetsCollected) where P: HasChildren<C1>, C1: HasChildren<C2>, C2: ResolvableRefs {
	for c in parent.children() {
		sub_pop(c, refs);
	}
}

fn sub_sub_res<P, C1, C2>(mut parent: P, refs: &TargetsCollected) -> P where P: e::Element + HasChildren<C1>, C1: e::Element + HasChildren<C2>, C2: ResolvableRefs {
	let new: Vec<_> = parent.children_mut().drain(..).map(|c| sub_res(c, refs)).collect();
	parent.children_mut().extend(new);
	parent
}

impl ResolvableRefs for c::StructuralSubElement {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::StructuralSubElement::*;
		match self {
			Title(e)        => sub_pop(&**e, refs),
			Subtitle(e)     => sub_pop(&**e, refs),
			Decoration(e)   => sub_pop(&**e, refs),
			Docinfo(e)      => sub_pop(&**e, refs),
			SubStructure(e) => e.populate_targets(refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
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
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::SubStructure::*;
		match self {
			Topic(e) => sub_pop(&**e, refs),
			Sidebar(e) => sub_pop(&**e, refs),
			Transition(e) => {
				// TODO
			},
			Section(e) => sub_pop(&**e, refs),
			BodyElement(e) => e.populate_targets(refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::SubStructure::*;
		match self {
			Topic(e) => sub_res(*e, refs).into(),
			Sidebar(e) => sub_res(*e, refs).into(),
			Transition(e) => Transition(e),
			Section(e) => sub_res(*e, refs).into(),
			BodyElement(e) => e.resolve_refs(refs).into(),
		}
	}
}

impl ResolvableRefs for c::BodyElement {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::BodyElement::*;
		match self {
			Paragraph(e) => sub_pop(&**e, refs),
			LiteralBlock(e) => sub_pop(&**e, refs),
			DoctestBlock(e) => sub_pop(&**e, refs),
			MathBlock(e) => {
				// TODO
			},
			Rubric(e) => sub_pop(&**e, refs),
			SubstitutionDefinition(e) => sub_pop(&**e, refs),
			Comment(e) => sub_pop(&**e, refs),
			Pending(e) => {
				// TODO
			},
			Target(e) => {
				// TODO
			},
			Raw(e) => {
				// TODO
			},
			Image(e) => {
				// TODO
			},
			Compound(e) => sub_pop(&**e, refs),
			Container(e) => sub_pop(&**e, refs),
			BulletList(e) => sub_sub_pop(&**e, refs),
			EnumeratedList(e) => sub_sub_pop(&**e, refs),
			DefinitionList(e) => sub_sub_pop(&**e, refs),
			FieldList(e) => sub_sub_pop(&**e, refs),
			OptionList(e) => sub_sub_pop(&**e, refs),
			LineBlock(e) => sub_pop(&**e, refs),
			BlockQuote(e) => sub_pop(&**e, refs),
			Admonition(e) => sub_pop(&**e, refs),
			Attention(e) => sub_pop(&**e, refs),
			Hint(e) => sub_pop(&**e, refs),
			Note(e) => sub_pop(&**e, refs),
			Caution(e) => sub_pop(&**e, refs),
			Danger(e) => sub_pop(&**e, refs),
			Error(e) => sub_pop(&**e, refs),
			Important(e) => sub_pop(&**e, refs),
			Tip(e) => sub_pop(&**e, refs),
			Warning(e) => sub_pop(&**e, refs),
			Footnote(e) => sub_pop(&**e, refs),
			Citation(e) => sub_pop(&**e, refs),
			SystemMessage(e) => sub_pop(&**e, refs),
			Figure(e) => sub_pop(&**e, refs),
			Table(e) => sub_pop(&**e, refs)
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::BodyElement::*;
		match self {
			Paragraph(e) => sub_res(*e, refs).into(),
			LiteralBlock(e) => sub_res(*e, refs).into(),
			DoctestBlock(e) => sub_res(*e, refs).into(),
			MathBlock(e) => MathBlock(e),
			Rubric(e) => sub_res(*e, refs).into(),
			SubstitutionDefinition(e) => sub_res(*e, refs).into(),
			Comment(e) => sub_res(*e, refs).into(),
			Pending(e) => Pending(e),
			Target(e) => Target(e),
			Raw(e) => Raw(e),
			Image(e) => Image(e),
			Compound(e) => sub_res(*e, refs).into(),
			Container(e) => sub_res(*e, refs).into(),
			BulletList(e) => sub_sub_res(*e, refs).into(),
			EnumeratedList(e) => sub_sub_res(*e, refs).into(),
			DefinitionList(e) => sub_sub_res(*e, refs).into(),
			FieldList(e) => sub_sub_res(*e, refs).into(),
			OptionList(e) => sub_sub_res(*e, refs).into(),
			LineBlock(e) => sub_res(*e, refs).into(),
			BlockQuote(e) => sub_res(*e, refs).into(),
			Admonition(e) => sub_res(*e, refs).into(),
			Attention(e) => sub_res(*e, refs).into(),
			Hint(e) => sub_res(*e, refs).into(),
			Note(e) => sub_res(*e, refs).into(),
			Caution(e) => sub_res(*e, refs).into(),
			Danger(e) => sub_res(*e, refs).into(),
			Error(e) => sub_res(*e, refs).into(),
			Important(e) => sub_res(*e, refs).into(),
			Tip(e) => sub_res(*e, refs).into(),
			Warning(e) => sub_res(*e, refs).into(),
			Footnote(e) => sub_res(*e, refs).into(),
			Citation(e) => sub_res(*e, refs).into(),
			SystemMessage(e) => sub_res(*e, refs).into(),
			Figure(e) => sub_res(*e, refs).into(),
			Table(e) => sub_res(*e, refs).into()
		}
	}
}

impl ResolvableRefs for c::BibliographicElement {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::BibliographicElement::*;
		match self {
			Author(e) => sub_pop(&**e, refs),
			Authors(e) => sub_pop(&**e, refs),
			Organization(e) => sub_pop(&**e, refs),
			Address(e) => sub_pop(&**e, refs),
			Contact(e) => sub_pop(&**e, refs),
			Version(e) => sub_pop(&**e, refs),
			Revision(e) => sub_pop(&**e, refs),
			Status(e) => sub_pop(&**e, refs),
			Date(e) => sub_pop(&**e, refs),
			Copyright(e) => sub_pop(&**e, refs),
			Field(e) => sub_pop(&**e, refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::BibliographicElement::*;
		match self {
			Author(e) => sub_res(*e, refs).into(),
			Authors(e) => sub_res(*e, refs).into(),
			Organization(e) => sub_res(*e, refs).into(),
			Address(e) => sub_res(*e, refs).into(),
			Contact(e) => sub_res(*e, refs).into(),
			Version(e) => sub_res(*e, refs).into(),
			Revision(e) => sub_res(*e, refs).into(),
			Status(e) => sub_res(*e, refs).into(),
			Date(e) => sub_res(*e, refs).into(),
			Copyright(e) => sub_res(*e, refs).into(),
			Field(e) => sub_res(*e, refs).into(),
		}
	}
}

impl ResolvableRefs for c::TextOrInlineElement {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::TextOrInlineElement::*;
		match self {
			c::TextOrInlineElement::String(e) => {
				// TODO
			},
			Emphasis(e) => sub_pop(&**e, refs),
			Strong(e) => sub_pop(&**e, refs),
			Literal(e) => sub_pop(&**e, refs),
			Reference(e) => sub_pop(&**e, refs),
			FootnoteReference(e) => sub_pop(&**e, refs),
			CitationReference(e) => sub_pop(&**e, refs),
			SubstitutionReference(e) => sub_pop(&**e, refs),
			TitleReference(e) => sub_pop(&**e, refs),
			Abbreviation(e) => sub_pop(&**e, refs),
			Acronym(e) => sub_pop(&**e, refs),
			Superscript(e) => sub_pop(&**e, refs),
			Subscript(e) => sub_pop(&**e, refs),
			Inline(e) => sub_pop(&**e, refs),
			Problematic(e) => sub_pop(&**e, refs),
			Generated(e) => sub_pop(&**e, refs),
			Math(e) => {
				// TODO
			},
			TargetInline(e) => {
				// TODO
			},
			RawInline(e) => {
				// TODO
			},
			ImageInline(e) => {
				// TODO
			}
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::TextOrInlineElement::*;
		match self {
			c::TextOrInlineElement::String(e) => c::TextOrInlineElement::String(e),
			Emphasis(e) => sub_res(*e, refs).into(),
			Strong(e) => sub_res(*e, refs).into(),
			Literal(e) => sub_res(*e, refs).into(),
			Reference(e) => sub_res(*e, refs).into(),
			FootnoteReference(e) => sub_res(*e, refs).into(),
			CitationReference(e) => sub_res(*e, refs).into(),
			SubstitutionReference(e) => sub_res(*e, refs).into(),
			TitleReference(e) => sub_res(*e, refs).into(),
			Abbreviation(e) => sub_res(*e, refs).into(),
			Acronym(e) => sub_res(*e, refs).into(),
			Superscript(e) => sub_res(*e, refs).into(),
			Subscript(e) => sub_res(*e, refs).into(),
			Inline(e) => sub_res(*e, refs).into(),
			Problematic(e) => sub_res(*e, refs).into(),
			Generated(e) => sub_res(*e, refs).into(),
			Math(e) => Math(e),
			TargetInline(e) => TargetInline(e),
			RawInline(e) => RawInline(e),
			ImageInline(e) => ImageInline(e)
		}
	}
}

impl ResolvableRefs for c::AuthorInfo {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::AuthorInfo::*;
		match self {
			Author(e) => sub_pop(&**e, refs),
			Organization(e) => sub_pop(&**e, refs),
			Address(e) => sub_pop(&**e, refs),
			Contact(e) => sub_pop(&**e, refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::AuthorInfo::*;
		match self {
			Author(e) => sub_res(*e, refs).into(),
			Organization(e) => sub_res(*e, refs).into(),
			Address(e) => sub_res(*e, refs).into(),
			Contact(e) => sub_res(*e, refs).into(),
		}
	}
}

impl ResolvableRefs for c::DecorationElement {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::DecorationElement::*;
		match self {
			Header(e) => sub_pop(&**e, refs),
			Footer(e) => sub_pop(&**e, refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::DecorationElement::*;
		match self {
			Header(e) => sub_res(*e, refs).into(),
			Footer(e) => sub_res(*e, refs).into(),
		}
	}
}

impl ResolvableRefs for c::SubTopic {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::SubTopic::*;
		match self {
			Title(e) => sub_pop(&**e, refs),
			BodyElement(e) => e.populate_targets(refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::SubTopic::*;
		match self {
			Title(e) => sub_res(*e, refs).into(),
			BodyElement(e) => e.resolve_refs(refs).into(),
		}
	}
}

impl ResolvableRefs for c::SubSidebar {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::SubSidebar::*;
		match self {
			Topic(e) => sub_pop(&**e, refs),
			Title(e) => sub_pop(&**e, refs),
			Subtitle(e) => sub_pop(&**e, refs),
			BodyElement(e) => e.populate_targets(refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::SubSidebar::*;
		match self {
			Topic(e) => sub_res(*e, refs).into(),
			Title(e) => sub_res(*e, refs).into(),
			Subtitle(e) => sub_res(*e, refs).into(),
			BodyElement(e) => e.resolve_refs(refs).into(),
		}
	}
}

impl ResolvableRefs for c::SubDLItem {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::SubDLItem::*;
		match self {
			Term(e) => sub_pop(&**e, refs),
			Classifier(e) => sub_pop(&**e, refs),
			Definition(e) => sub_pop(&**e, refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::SubDLItem::*;
		match self {
			Term(e) => sub_res(*e, refs).into(),
			Classifier(e) => sub_res(*e, refs).into(),
			Definition(e) => sub_res(*e, refs).into(),
		}
	}
}

impl ResolvableRefs for c::SubField {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::SubField::*;
		match self {
			FieldName(e) => sub_pop(&**e, refs),
			FieldBody(e) => sub_pop(&**e, refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::SubField::*;
		match self {
			FieldName(e) => sub_res(*e, refs).into(),
			FieldBody(e) => sub_res(*e, refs).into(),
		}
	}
}

impl ResolvableRefs for c::SubOptionListItem {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::SubOptionListItem::*;
		match self {
			OptionGroup(e) => sub_sub_pop(&**e, refs),
			Description(e) => sub_pop(&**e, refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::SubOptionListItem::*;
		match self {
			OptionGroup(e) => sub_sub_res(*e, refs).into(),
			Description(e) => sub_res(*e, refs).into(),
		}
	}
}

impl ResolvableRefs for c::SubOption {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::SubOption::*;
		match self {
			OptionString(e) => {
				// TODO
			},
			OptionArgument(e) => {
				// TODO
			},
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::SubOption::*;
		match self {
			OptionString(e) => OptionString(e),
			OptionArgument(e) => OptionArgument(e),
		}
	}
}

impl ResolvableRefs for c::SubLineBlock {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::SubLineBlock::*;
		match self {
			LineBlock(e) => sub_pop(&**e, refs),
			Line(e) => sub_pop(&**e, refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::SubLineBlock::*;
		match self {
			LineBlock(e) => sub_res(*e, refs).into(),
			Line(e) => sub_res(*e, refs).into(),
		}
	}
}

impl ResolvableRefs for c::SubBlockQuote {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::SubBlockQuote::*;
		match self {
			Attribution(e) => sub_pop(&**e, refs),
			BodyElement(e) => e.populate_targets(refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::SubBlockQuote::*;
		match self {
			Attribution(e) => sub_res(*e, refs).into(),
			BodyElement(e) => e.resolve_refs(refs).into(),
		}
	}
}

impl ResolvableRefs for c::SubFootnote {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::SubFootnote::*;
		match self {
			Label(e) => sub_pop(&**e, refs),
			BodyElement(e) => e.populate_targets(refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::SubFootnote::*;
		match self {
			Label(e) => sub_res(*e, refs).into(),
			BodyElement(e) => e.resolve_refs(refs).into(),
		}
	}
}

impl ResolvableRefs for c::SubFigure {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::SubFigure::*;
		match self {
			Caption(e) => sub_pop(&**e, refs),
			Legend(e) => sub_pop(&**e, refs),
			BodyElement(e) => e.populate_targets(refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::SubFigure::*;
		match self {
			Caption(e) => sub_res(*e, refs).into(),
			Legend(e) => sub_res(*e, refs).into(),
			BodyElement(e) => e.resolve_refs(refs).into(),
		}
	}
}

impl ResolvableRefs for c::SubTable {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::SubTable::*;
		match self {
			Title(e) => sub_pop(&**e, refs),
			TableGroup(e) => sub_pop(&**e, refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::SubTable::*;
		match self {
			Title(e) => sub_res(*e, refs).into(),
			TableGroup(e) => sub_res(*e, refs).into(),
		}
	}
}

impl ResolvableRefs for c::SubTableGroup {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::SubTableGroup::*;
		match self {
			TableColspec(e) => {
				// TODO
			},
			TableHead(e) => {
				for c in e.children() {
					sub_sub_pop(c, refs);
				}
			},
			TableBody(e) => {
				for c in e.children() {
					sub_sub_pop(c, refs);
				}
			},
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Self {
		use c::SubTableGroup::*;
		match self {
			TableColspec(e) => TableColspec(e),
			TableHead(mut e) => {
				let new: Vec<_> = e.children_mut().drain(..).map(|c| sub_sub_res(c, refs)).collect();
				e.children_mut().extend(new);
				TableHead(e)
			},
			TableBody(mut e) => {
				let new: Vec<_> = e.children_mut().drain(..).map(|c| sub_sub_res(c, refs)).collect();
				e.children_mut().extend(new);
				TableBody(e)
			},
		}
	}
}