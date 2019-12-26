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

TODO: continue documenting how it’s done via https://repo.or.cz/docutils.git/blob/HEAD:/docutils/docutils/transforms/references.py
*/

use std::collections::HashMap;

use document_tree::{
	url::Url,
	Document,
	HasChildren,
	attribute_types::NameToken,
	elements::{self as e, Element},
	element_categories as c,
	extra_attributes::ExtraAttributes,
};


#[derive(Debug)]
enum NamedTargetType {
	NumberedFootnote(usize),
	LabeledFootnote(usize),
	Citation,
	InternalLink,
	ExternalLink(Url),
	IndirectLink(NameToken),
	SectionTitle,
}
impl NamedTargetType {
	fn is_implicit_target(&self) -> bool {
		match self {
			NamedTargetType::SectionTitle => true,
			_ => false,
		}
	}
}

#[derive(Clone, Debug)]
struct Substitution {
	content: Vec<c::TextOrInlineElement>,
	/// If true and the sibling before the reference is a text node,
	/// the text node gets right-trimmed. 
	ltrim: bool,
	/// Same as `ltrim` with the sibling after the reference.
	rtrim: bool,
}

#[derive(Default, Debug)]
struct TargetsCollected {
	named_targets: HashMap<NameToken, NamedTargetType>,
	substitutions: HashMap<NameToken, Substitution>,
	normalized_substitutions: HashMap<String, Substitution>,
}
impl TargetsCollected {
	fn target_url<'t>(self: &'t TargetsCollected, refname: &[NameToken]) -> Option<&'t Url> {
		// TODO: Check if the target would expand circularly
		if refname.len() != 1 {
			panic!("Expected exactly one name in a reference.");
		}
		let name = refname[0].clone();
		match self.named_targets.get(&name)? {
			NamedTargetType::ExternalLink(url) => Some(url),
			_ => unimplemented!(),
		}
	}
	
	fn substitution<'t>(self: &'t TargetsCollected, refname: &[NameToken]) -> Option<&'t Substitution> {
		// TODO: Check if the substitution would expand circularly
		if refname.len() != 1 {
			panic!("Expected exactly one name in a substitution reference.");
		}
		let name = refname[0].clone();
		self.substitutions.get(&name).or_else(|| {
			self.normalized_substitutions.get(&name.0.to_lowercase())
		})
	}
}

trait ResolvableRefs {
	fn populate_targets(&self, refs: &mut TargetsCollected);
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> where Self: Sized;
}

pub fn resolve_references(mut doc: Document) -> Document {
	let mut references: TargetsCollected = Default::default();
	for c in doc.children() {
		c.populate_targets(&mut references);
	}
	let new: Vec<_> = doc.children_mut().drain(..).flat_map(|c| c.resolve_refs(&references)).collect();
	Document::with_children(new)
}

fn sub_pop<P, C>(parent: &P, refs: &mut TargetsCollected) where P: HasChildren<C>, C: ResolvableRefs {
	for c in parent.children() {
		c.populate_targets(refs);
	}
}

fn sub_res<P, C>(mut parent: P, refs: &TargetsCollected) -> P where P: e::Element + HasChildren<C>, C: ResolvableRefs {
	let new: Vec<_> = parent.children_mut().drain(..).flat_map(|c| c.resolve_refs(refs)).collect();
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
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::StructuralSubElement::*;
		vec![match self {
			Title(e)        => sub_res(*e, refs).into(),
			Subtitle(e)     => sub_res(*e, refs).into(),
			Decoration(e)   => sub_res(*e, refs).into(),
			Docinfo(e)      => sub_res(*e, refs).into(),
			SubStructure(e) => return e.resolve_refs(refs).drain(..).map(Into::into).collect(),
		}]
	}
}

impl ResolvableRefs for c::SubStructure {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::SubStructure::*;
		match self {
			Topic(e) => sub_pop(&**e, refs),
			Sidebar(e) => sub_pop(&**e, refs),
			Transition(_) => {},
			Section(e) => sub_pop(&**e, refs),
			BodyElement(e) => e.populate_targets(refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::SubStructure::*;
		vec![match self {
			Topic(e) => sub_res(*e, refs).into(),
			Sidebar(e) => sub_res(*e, refs).into(),
			Transition(e) => Transition(e),
			Section(e) => sub_res(*e, refs).into(),
			BodyElement(e) => return e.resolve_refs(refs).drain(..).map(Into::into).collect(),
		}]
	}
}

impl ResolvableRefs for c::BodyElement {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::BodyElement::*;
		match self {
			Paragraph(e) => sub_pop(&**e, refs),
			LiteralBlock(e) => sub_pop(&**e, refs),
			DoctestBlock(e) => sub_pop(&**e, refs),
			MathBlock(_) => {},
			Rubric(e) => sub_pop(&**e, refs),
			SubstitutionDefinition(e) => {
				let subst = Substitution {
					content: e.children().clone(),
					ltrim: e.extra().ltrim,
					rtrim: e.extra().rtrim
				};
				for name in e.names() {
					if refs.substitutions.contains_key(name) {
						// TODO: Duplicate substitution name (level 3 system message).
					}
					// Intentionally overriding any previous values.
					refs.substitutions.insert(name.clone(), subst.clone());
					refs.normalized_substitutions.insert(name.0.to_lowercase(), subst.clone());
				}
			},
			Comment(_) => {},
			Pending(_) => {
				unimplemented!();
			},
			Target(e) => {
				if let Some(uri) = &e.extra().refuri {
					for name in e.names() {
						refs.named_targets.insert(name.clone(), NamedTargetType::ExternalLink(uri.clone()));
					}
				}
				// TODO: as is, people can only refer to the target directly containing the URL.
				// add refid and refnames to some HashMap and follow those later.
			},
			Raw(_) => {},
			Image(_) => {},
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
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::BodyElement::*;
		vec![match self {
			Paragraph(e) => sub_res(*e, refs).into(),
			LiteralBlock(e) => sub_res(*e, refs).into(),
			DoctestBlock(e) => sub_res(*e, refs).into(),
			MathBlock(e) => MathBlock(e),
			Rubric(e) => sub_res(*e, refs).into(),
			SubstitutionDefinition(_) => return vec![],
			Comment(e) => Comment(e),
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
		}]
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
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::BibliographicElement::*;
		vec![match self {
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
		}]
	}
}

impl ResolvableRefs for c::TextOrInlineElement {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::TextOrInlineElement::*;
		match self {
			String(_) => {},
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
			Math(_) => {},
			TargetInline(_) => {
				unimplemented!();
			},
			RawInline(_) => {},
			ImageInline(_) => {}
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::TextOrInlineElement::*;
		vec![match self {
			String(e) => String(e),
			Emphasis(e) => sub_res(*e, refs).into(),
			Strong(e) => sub_res(*e, refs).into(),
			Literal(e) => sub_res(*e, refs).into(),
			Reference(mut e) => {
				if e.extra().refuri.is_none() {
					if let Some(uri) = refs.target_url(&e.extra().refname) {
						e.extra_mut().refuri = Some(uri.clone());
					}
				}
				(*e).into()
			},
			FootnoteReference(e) => sub_res(*e, refs).into(),
			CitationReference(e) => sub_res(*e, refs).into(),
			SubstitutionReference(e) => match refs.substitution(&e.extra().refname) {
				Some(Substitution {content, ltrim, rtrim}) => {
					// (level 3 system message).
					// TODO: ltrim and rtrim.
					if *ltrim || *rtrim {
						dbg!(content, ltrim, rtrim);
					}
					return content.clone()
				},
				None => {
					// Undefined substitution name (level 3 system message).
					// TODO: This replaces the reference by a Problematic node.
					// The corresponding SystemMessage node should go in a generated
					// section with class "system-messages" at the end of the document.
					use document_tree::Problematic;
					let mut replacement: Box<Problematic> = Box::new(Default::default());
					replacement.children_mut().push(
						c::TextOrInlineElement::String(Box::new(format!("|{}|", e.extra().refname[0].0)))
					);
					// TODO: Create an ID for replacement for the system_message to reference.
					// TODO: replacement.refid pointing to the system_message.
					Problematic(replacement)
				}
			},
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
		}]
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
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::AuthorInfo::*;
		vec![match self {
			Author(e) => sub_res(*e, refs).into(),
			Organization(e) => sub_res(*e, refs).into(),
			Address(e) => sub_res(*e, refs).into(),
			Contact(e) => sub_res(*e, refs).into(),
		}]
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
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::DecorationElement::*;
		vec![match self {
			Header(e) => sub_res(*e, refs).into(),
			Footer(e) => sub_res(*e, refs).into(),
		}]
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
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::SubTopic::*;
		match self {
			Title(e) => vec![sub_res(*e, refs).into()],
			BodyElement(e) => e.resolve_refs(refs).drain(..).map(Into::into).collect(),
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
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::SubSidebar::*;
		vec![match self {
			Topic(e) => sub_res(*e, refs).into(),
			Title(e) => sub_res(*e, refs).into(),
			Subtitle(e) => sub_res(*e, refs).into(),
			BodyElement(e) => return e.resolve_refs(refs).drain(..).map(Into::into).collect(),
		}]
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
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::SubDLItem::*;
		vec![match self {
			Term(e) => sub_res(*e, refs).into(),
			Classifier(e) => sub_res(*e, refs).into(),
			Definition(e) => sub_res(*e, refs).into(),
		}]
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
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::SubField::*;
		vec![match self {
			FieldName(e) => sub_res(*e, refs).into(),
			FieldBody(e) => sub_res(*e, refs).into(),
		}]
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
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::SubOptionListItem::*;
		vec![match self {
			OptionGroup(e) => sub_sub_res(*e, refs).into(),
			Description(e) => sub_res(*e, refs).into(),
		}]
	}
}

impl ResolvableRefs for c::SubOption {
	fn populate_targets(&self, _: &mut TargetsCollected) {}
	fn resolve_refs(self, _: &TargetsCollected) -> Vec<Self> { vec![self] }
}

impl ResolvableRefs for c::SubLineBlock {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::SubLineBlock::*;
		match self {
			LineBlock(e) => sub_pop(&**e, refs),
			Line(e) => sub_pop(&**e, refs),
		}
	}
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::SubLineBlock::*;
		vec![match self {
			LineBlock(e) => sub_res(*e, refs).into(),
			Line(e) => sub_res(*e, refs).into(),
		}]
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
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::SubBlockQuote::*;
		match self {
			Attribution(e) => vec![sub_res(*e, refs).into()],
			BodyElement(e) => e.resolve_refs(refs).drain(..).map(Into::into).collect(),
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
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::SubFootnote::*;
		match self {
			Label(e) => vec![sub_res(*e, refs).into()],
			BodyElement(e) => e.resolve_refs(refs).drain(..).map(Into::into).collect(),
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
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::SubFigure::*;
		vec![match self {
			Caption(e) => sub_res(*e, refs).into(),
			Legend(e) => sub_res(*e, refs).into(),
			BodyElement(e) => return e.resolve_refs(refs).drain(..).map(Into::into).collect(),
		}]
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
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::SubTable::*;
		vec![match self {
			Title(e) => sub_res(*e, refs).into(),
			TableGroup(e) => sub_res(*e, refs).into(),
		}]
	}
}

impl ResolvableRefs for c::SubTableGroup {
	fn populate_targets(&self, refs: &mut TargetsCollected) {
		use c::SubTableGroup::*;
		match self {
			TableColspec(_) => {
				unimplemented!();
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
	fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
		use c::SubTableGroup::*;
		vec![match self {
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
		}]
	}
}
