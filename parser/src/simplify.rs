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

use std::{collections::HashMap, num::NonZero};

use document_tree::{
    Document, HasChildren, LabelledFootnote as _,
    attribute_types::{AutoFootnoteType, ID, NameToken},
    element_categories as c,
    elements::{self as e, Element},
    extra_attributes::ExtraAttributes,
    url::Url,
};

#[derive(Debug)]
#[allow(dead_code)]
enum NamedTargetType {
    // TODO: symbol footnotes
    NumberedFootnote(NonZero<usize>),
    LabeledFootnote(NonZero<usize>),
    Citation,
    InternalLink,
    ExternalLink(Url),
    IndirectLink(NameToken),
    SectionTitle,
}
impl NamedTargetType {
    #[allow(dead_code)]
    /// See <https://docutils.sourceforge.io/docs/ref/rst/restructuredtext.html#implicit-hyperlink-targets>
    fn is_implicit_target(&self) -> bool {
        use NamedTargetType as T;
        matches!(
            self,
            T::SectionTitle | T::NumberedFootnote(_) | T::LabeledFootnote(_) | T::Citation
        )
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
    // TODO: symbol ones only need counters, not vecs!
    footnote_refs_number: HashMap<ID, NonZero<usize>>,
    footnote_refs_symbol: HashMap<ID, NonZero<usize>>,
    footnotes_number: HashMap<ID, NonZero<usize>>,
    footnotes_symbol: HashMap<ID, NonZero<usize>>,
}
impl TargetsCollected {
    fn target_url<'t>(self: &'t TargetsCollected, refname: &[NameToken]) -> Option<&'t Url> {
        // TODO: Check if the target would expand circularly
        assert!(
            refname.len() == 1,
            "Expected exactly one name in a reference."
        );
        let name = refname[0].clone();
        match self.named_targets.get(&name)? {
            NamedTargetType::ExternalLink(url) => Some(url),
            _ => unimplemented!(),
        }
    }

    fn substitution<'t>(
        self: &'t TargetsCollected,
        refname: &[NameToken],
    ) -> Option<&'t Substitution> {
        // TODO: Check if the substitution would expand circularly
        assert!(
            refname.len() == 1,
            "Expected exactly one name in a substitution reference."
        );
        let name = refname[0].clone();
        self.substitutions
            .get(&name)
            .or_else(|| self.normalized_substitutions.get(&name.0.to_lowercase()))
    }

    fn next_footnote(
        &mut self,
        typ: AutoFootnoteType,
        is_ref: bool,
        named: bool,
    ) -> NonZero<usize> {
        // Auto-numbered named footnotes get the *lowest* available number.
        // Auto-numbered anonymous footnotes get the *next* available number.
        // See <https://docutils.sourceforge.io/docs/ref/rst/restructuredtext.html#mixed-manual-and-auto-numbered-footnotes>
        let it = match (typ, is_ref) {
            (AutoFootnoteType::Number, true) => &mut self.footnote_refs_number,
            (AutoFootnoteType::Symbol, true) => &mut self.footnote_refs_symbol,
            (AutoFootnoteType::Number, false) => &mut self.footnotes_number,
            (AutoFootnoteType::Symbol, false) => &mut self.footnotes_symbol,
        }
        .values()
        .copied();
        if named { it.min() } else { it.max() }
            .map_or(NonZero::new(1usize).unwrap(), |n| n.saturating_add(1))
    }
}

trait ResolvableRefs {
    /// Populate `TargetsCollected`
    fn populate_targets(&self, refs: &mut TargetsCollected);
    /// Transform `self` based on the complete `TargetsCollected`
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self>
    where
        Self: Sized;
}

pub fn resolve_references(mut doc: Document) -> Document {
    let mut references = TargetsCollected::default();
    for c in doc.children() {
        c.populate_targets(&mut references);
    }
    let new: Vec<_> = doc
        .children_mut()
        .drain(..)
        .flat_map(|c| c.resolve_refs(&references))
        .collect();
    Document::with_children(new)
}

fn sub_pop<P, C>(parent: &P, refs: &mut TargetsCollected)
where
    P: HasChildren<C>,
    C: ResolvableRefs,
{
    for c in parent.children() {
        c.populate_targets(refs);
    }
}

fn sub_res<P, C>(mut parent: P, refs: &TargetsCollected) -> P
where
    P: e::Element + HasChildren<C>,
    C: ResolvableRefs,
{
    let new: Vec<_> = parent
        .children_mut()
        .drain(..)
        .flat_map(|c| c.resolve_refs(refs))
        .collect();
    parent.children_mut().extend(new);
    parent
}

fn sub_sub_pop<P, C1, C2>(parent: &P, refs: &mut TargetsCollected)
where
    P: HasChildren<C1>,
    C1: HasChildren<C2>,
    C2: ResolvableRefs,
{
    for c in parent.children() {
        sub_pop(c, refs);
    }
}

fn sub_sub_res<P, C1, C2>(mut parent: P, refs: &TargetsCollected) -> P
where
    P: e::Element + HasChildren<C1>,
    C1: e::Element + HasChildren<C2>,
    C2: ResolvableRefs,
{
    let new: Vec<_> = parent
        .children_mut()
        .drain(..)
        .map(|c| sub_res(c, refs))
        .collect();
    parent.children_mut().extend(new);
    parent
}

impl ResolvableRefs for c::StructuralSubElement {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::StructuralSubElement as S;
        match self {
            S::Title(e) => sub_pop(e.as_ref(), refs),
            S::Subtitle(e) => sub_pop(e.as_ref(), refs),
            S::Decoration(e) => sub_pop(e.as_ref(), refs),
            S::Docinfo(e) => sub_pop(e.as_ref(), refs),
            S::SubStructure(e) => e.populate_targets(refs),
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::StructuralSubElement as S;
        vec![match self {
            S::Title(e) => sub_res(*e, refs).into(),
            S::Subtitle(e) => sub_res(*e, refs).into(),
            S::Decoration(e) => sub_res(*e, refs).into(),
            S::Docinfo(e) => sub_res(*e, refs).into(),
            S::SubStructure(e) => return e.resolve_refs(refs).drain(..).map(Into::into).collect(),
        }]
    }
}

impl ResolvableRefs for c::SubStructure {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::SubStructure as S;
        match self {
            S::Topic(e) => sub_pop(e.as_ref(), refs),
            S::Sidebar(e) => sub_pop(e.as_ref(), refs),
            S::Transition(_) => {}
            S::Section(e) => sub_pop(e.as_ref(), refs),
            S::BodyElement(e) => e.populate_targets(refs),
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::SubStructure as S;
        vec![match self {
            S::Topic(e) => sub_res(*e, refs).into(),
            S::Sidebar(e) => sub_res(*e, refs).into(),
            S::Transition(e) => S::Transition(e),
            S::Section(e) => sub_res(*e, refs).into(),
            S::BodyElement(e) => return e.resolve_refs(refs).drain(..).map(Into::into).collect(),
        }]
    }
}

impl ResolvableRefs for c::BodyElement {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::BodyElement as B;
        match self {
            B::Paragraph(e) => sub_pop(e.as_ref(), refs),
            B::LiteralBlock(e) => sub_pop(e.as_ref(), refs),
            B::DoctestBlock(e) => sub_pop(e.as_ref(), refs),
            B::Rubric(e) => sub_pop(e.as_ref(), refs),
            B::SubstitutionDefinition(e) => {
                let subst = Substitution {
                    content: e.children().clone(),
                    ltrim: e.extra().ltrim,
                    rtrim: e.extra().rtrim,
                };
                for name in e.names() {
                    if refs.substitutions.contains_key(name) {
                        // TODO: Duplicate substitution name (level 3 system message).
                    }
                    // Intentionally overriding any previous values.
                    refs.substitutions.insert(name.clone(), subst.clone());
                    refs.normalized_substitutions
                        .insert(name.0.to_lowercase(), subst.clone());
                }
            }
            B::Pending(_) => {
                unimplemented!();
            }
            B::Target(e) => {
                if let Some(uri) = &e.extra().refuri {
                    for name in e.names() {
                        refs.named_targets
                            .insert(name.clone(), NamedTargetType::ExternalLink(uri.clone()));
                    }
                }
                // TODO: as is, people can only refer to the target directly containing the URL.
                // add refid and refnames to some HashMap and follow those later.
            }
            B::Compound(e) => sub_pop(e.as_ref(), refs),
            B::Container(e) => sub_pop(e.as_ref(), refs),
            B::BulletList(e) => sub_sub_pop(e.as_ref(), refs),
            B::EnumeratedList(e) => sub_sub_pop(e.as_ref(), refs),
            B::DefinitionList(e) => sub_sub_pop(e.as_ref(), refs),
            B::FieldList(e) => sub_sub_pop(e.as_ref(), refs),
            B::OptionList(e) => sub_sub_pop(e.as_ref(), refs),
            B::LineBlock(e) => sub_pop(e.as_ref(), refs),
            B::BlockQuote(e) => sub_pop(e.as_ref(), refs),
            B::Admonition(e) => sub_pop(e.as_ref(), refs),
            B::Attention(e) => sub_pop(e.as_ref(), refs),
            B::Hint(e) => sub_pop(e.as_ref(), refs),
            B::Note(e) => sub_pop(e.as_ref(), refs),
            B::Caution(e) => sub_pop(e.as_ref(), refs),
            B::Danger(e) => sub_pop(e.as_ref(), refs),
            B::Error(e) => sub_pop(e.as_ref(), refs),
            B::Important(e) => sub_pop(e.as_ref(), refs),
            B::Tip(e) => sub_pop(e.as_ref(), refs),
            B::Warning(e) => sub_pop(e.as_ref(), refs),
            B::Footnote(e) => {
                // TODO: dedupe
                /*
                1. (here) add auto-id and running count to “ids” of footnote references and footnotes
                2. see below
                */
                let n = match e.extra().auto {
                    Some(t @ AutoFootnoteType::Number) => {
                        let n = refs.next_footnote(t, false, !e.names().is_empty());
                        for name in e.names() {
                            refs.named_targets
                                .insert(name.clone(), NamedTargetType::LabeledFootnote(n));
                        }
                        Some(n)
                    }
                    Some(t @ AutoFootnoteType::Symbol) => Some(refs.next_footnote(t, false, false)),
                    None => e.get_label().ok().and_then(|l| l.parse().ok()),
                };
                if let Some(n) = n {
                    for id in e.ids() {
                        match e.extra().auto {
                            Some(AutoFootnoteType::Symbol) => {
                                refs.footnotes_symbol.insert(id.clone(), n)
                            }
                            _ => refs.footnotes_number.insert(id.clone(), n),
                        };
                    }
                }
                sub_pop(e.as_ref(), refs);
            }
            B::Citation(e) => sub_pop(e.as_ref(), refs),
            B::SystemMessage(e) => sub_pop(e.as_ref(), refs),
            B::Figure(e) => sub_pop(e.as_ref(), refs),
            B::Table(e) => sub_pop(e.as_ref(), refs),
            B::MathBlock(_) | B::Comment(_) | B::Raw(_) | B::Image(_) => {}
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::BodyElement as B;
        vec![match self {
            B::Paragraph(e) => sub_res(*e, refs).into(),
            B::LiteralBlock(e) => sub_res(*e, refs).into(),
            B::DoctestBlock(e) => sub_res(*e, refs).into(),
            B::MathBlock(e) => B::MathBlock(e),
            B::Rubric(e) => sub_res(*e, refs).into(),
            B::SubstitutionDefinition(_) => return vec![],
            B::Comment(e) => B::Comment(e),
            B::Pending(e) => B::Pending(e),
            B::Target(e) => B::Target(e),
            B::Raw(e) => B::Raw(e),
            B::Image(e) => B::Image(e),
            B::Compound(e) => sub_res(*e, refs).into(),
            B::Container(e) => sub_res(*e, refs).into(),
            B::BulletList(e) => sub_sub_res(*e, refs).into(),
            B::EnumeratedList(e) => sub_sub_res(*e, refs).into(),
            B::DefinitionList(e) => sub_sub_res(*e, refs).into(),
            B::FieldList(e) => sub_sub_res(*e, refs).into(),
            B::OptionList(e) => sub_sub_res(*e, refs).into(),
            B::LineBlock(e) => sub_res(*e, refs).into(),
            B::BlockQuote(e) => sub_res(*e, refs).into(),
            B::Admonition(e) => sub_res(*e, refs).into(),
            B::Attention(e) => sub_res(*e, refs).into(),
            B::Hint(e) => sub_res(*e, refs).into(),
            B::Note(e) => sub_res(*e, refs).into(),
            B::Caution(e) => sub_res(*e, refs).into(),
            B::Danger(e) => sub_res(*e, refs).into(),
            B::Error(e) => sub_res(*e, refs).into(),
            B::Important(e) => sub_res(*e, refs).into(),
            B::Tip(e) => sub_res(*e, refs).into(),
            B::Warning(e) => sub_res(*e, refs).into(),
            B::Footnote(mut e) => {
                // TODO: dedupe
                /* TODO: https://docutils.sourceforge.io/docs/ref/doctree.html#footnote-reference
                1. see above
                2. (in resolve_refs) set `footnote_reference[refid]`s, `footnote[backref]`s and `footnote>label`
                */
                if e.get_label().is_err() {
                    let label = e
                        .ids()
                        .iter()
                        .find_map(|id| match e.extra().auto {
                            Some(AutoFootnoteType::Symbol) => refs.footnotes_symbol.get(id),
                            _ => refs.footnotes_number.get(id),
                        })
                        .map_or_else(|| "???".into(), ToString::to_string);
                    e.children_mut()
                        .insert(0, e::Label::with_children(vec![label.into()]).into());
                }
                sub_res(*e, refs).into()
            }
            B::Citation(e) => sub_res(*e, refs).into(),
            B::SystemMessage(e) => sub_res(*e, refs).into(),
            B::Figure(e) => sub_res(*e, refs).into(),
            B::Table(e) => sub_res(*e, refs).into(),
        }]
    }
}

impl ResolvableRefs for c::BibliographicElement {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::BibliographicElement as B;
        match self {
            B::Author(e) => sub_pop(e.as_ref(), refs),
            B::Authors(e) => sub_pop(e.as_ref(), refs),
            B::Organization(e) => sub_pop(e.as_ref(), refs),
            B::Address(e) => sub_pop(e.as_ref(), refs),
            B::Contact(e) => sub_pop(e.as_ref(), refs),
            B::Version(e) => sub_pop(e.as_ref(), refs),
            B::Revision(e) => sub_pop(e.as_ref(), refs),
            B::Status(e) => sub_pop(e.as_ref(), refs),
            B::Date(e) => sub_pop(e.as_ref(), refs),
            B::Copyright(e) => sub_pop(e.as_ref(), refs),
            B::Field(e) => sub_pop(e.as_ref(), refs),
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::BibliographicElement as B;
        vec![match self {
            B::Author(e) => sub_res(*e, refs).into(),
            B::Authors(e) => sub_res(*e, refs).into(),
            B::Organization(e) => sub_res(*e, refs).into(),
            B::Address(e) => sub_res(*e, refs).into(),
            B::Contact(e) => sub_res(*e, refs).into(),
            B::Version(e) => sub_res(*e, refs).into(),
            B::Revision(e) => sub_res(*e, refs).into(),
            B::Status(e) => sub_res(*e, refs).into(),
            B::Date(e) => sub_res(*e, refs).into(),
            B::Copyright(e) => sub_res(*e, refs).into(),
            B::Field(e) => sub_res(*e, refs).into(),
        }]
    }
}

impl ResolvableRefs for c::TextOrInlineElement {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::TextOrInlineElement as T;
        match self {
            T::Emphasis(e) => sub_pop(e.as_ref(), refs),
            T::Strong(e) => sub_pop(e.as_ref(), refs),
            T::Reference(e) => sub_pop(e.as_ref(), refs),
            T::FootnoteReference(e) => {
                // TODO: dedupe
                let n = match e.extra().auto {
                    Some(t @ AutoFootnoteType::Number) => {
                        let n = refs.next_footnote(t, true, !e.names().is_empty());
                        for name in e.names() {
                            refs.named_targets
                                .insert(name.clone(), NamedTargetType::LabeledFootnote(n));
                        }
                        Some(n)
                    }
                    Some(t @ AutoFootnoteType::Symbol) => Some(refs.next_footnote(t, true, false)),
                    None => e.get_label().ok().and_then(|l| l.parse().ok()),
                };
                if let Some(n) = n {
                    for id in e.ids() {
                        match e.extra().auto {
                            Some(AutoFootnoteType::Symbol) => {
                                refs.footnote_refs_symbol.insert(id.clone(), n)
                            }
                            _ => refs.footnote_refs_number.insert(id.clone(), n),
                        };
                    }
                }
                sub_pop(e.as_ref(), refs);
            }
            T::CitationReference(e) => sub_pop(e.as_ref(), refs),
            T::SubstitutionReference(e) => sub_pop(e.as_ref(), refs),
            T::TitleReference(e) => sub_pop(e.as_ref(), refs),
            T::Abbreviation(e) => sub_pop(e.as_ref(), refs),
            T::Acronym(e) => sub_pop(e.as_ref(), refs),
            T::Superscript(e) => sub_pop(e.as_ref(), refs),
            T::Subscript(e) => sub_pop(e.as_ref(), refs),
            T::Inline(e) => sub_pop(e.as_ref(), refs),
            T::Problematic(e) => sub_pop(e.as_ref(), refs),
            T::Generated(e) => sub_pop(e.as_ref(), refs),
            T::TargetInline(_) => {
                unimplemented!();
            }
            T::String(_) | T::Literal(_) | T::Math(_) | T::RawInline(_) | T::ImageInline(_) => {}
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::TextOrInlineElement as T;
        use document_tree::Problematic;

        vec![match self {
            T::String(e) => T::String(e),
            T::Emphasis(e) => sub_res(*e, refs).into(),
            T::Strong(e) => sub_res(*e, refs).into(),
            T::Literal(e) => T::Literal(e),
            T::Reference(mut e) => {
                if e.extra().refuri.is_none() {
                    if let Some(uri) = refs.target_url(&e.extra().refname) {
                        e.extra_mut().refuri = Some(uri.clone());
                    }
                }
                (*e).into()
            }
            T::FootnoteReference(mut e) => {
                // TODO: dedupe
                // https://docutils.sourceforge.io/docs/ref/doctree.html#footnote-reference
                let n = e
                    .ids()
                    .iter()
                    .find_map(|id| match e.extra().auto {
                        Some(AutoFootnoteType::Symbol) => refs.footnote_refs_symbol.get(id),
                        _ => refs.footnote_refs_number.get(id),
                    })
                    .expect("Footnote reference without id");
                e.extra_mut().refid = Some(ID(format!("footnote-{n}")));
                if e.get_label().is_err() {
                    e.children_mut().insert(0, n.to_string().into());
                }
                sub_res(*e, refs).into()
            }
            T::CitationReference(e) => sub_res(*e, refs).into(),
            T::SubstitutionReference(e) => {
                if let Some(Substitution {
                    content,
                    ltrim,
                    rtrim,
                }) = refs.substitution(&e.extra().refname)
                {
                    // (level 3 system message).
                    // TODO: ltrim and rtrim.
                    if *ltrim || *rtrim {
                        dbg!(content, ltrim, rtrim);
                    }
                    return content.clone();
                }
                // Undefined substitution name (level 3 system message).
                // TODO: This replaces the reference by a Problematic node.
                // The corresponding SystemMessage node should go in a generated
                // section with class "system-messages" at the end of the document.
                let mut replacement: Box<Problematic> = Box::default();
                replacement
                    .children_mut()
                    .push(c::TextOrInlineElement::String(Box::new(format!(
                        "|{}|",
                        e.extra().refname[0].0
                    ))));
                // TODO: Create an ID for replacement for the system_message to reference.
                // TODO: replacement.refid pointing to the system_message.
                T::Problematic(replacement)
            }
            T::TitleReference(e) => sub_res(*e, refs).into(),
            T::Abbreviation(e) => sub_res(*e, refs).into(),
            T::Acronym(e) => sub_res(*e, refs).into(),
            T::Superscript(e) => sub_res(*e, refs).into(),
            T::Subscript(e) => sub_res(*e, refs).into(),
            T::Inline(e) => sub_res(*e, refs).into(),
            T::Problematic(e) => sub_res(*e, refs).into(),
            T::Generated(e) => sub_res(*e, refs).into(),
            T::Math(e) => T::Math(e),
            T::TargetInline(e) => T::TargetInline(e),
            T::RawInline(e) => T::RawInline(e),
            T::ImageInline(e) => T::ImageInline(e),
        }]
    }
}

impl ResolvableRefs for c::AuthorInfo {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::AuthorInfo as A;
        match self {
            A::Author(e) => sub_pop(e.as_ref(), refs),
            A::Organization(e) => sub_pop(e.as_ref(), refs),
            A::Address(e) => sub_pop(e.as_ref(), refs),
            A::Contact(e) => sub_pop(e.as_ref(), refs),
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::AuthorInfo as A;
        vec![match self {
            A::Author(e) => sub_res(*e, refs).into(),
            A::Organization(e) => sub_res(*e, refs).into(),
            A::Address(e) => sub_res(*e, refs).into(),
            A::Contact(e) => sub_res(*e, refs).into(),
        }]
    }
}

impl ResolvableRefs for c::DecorationElement {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::DecorationElement::{Footer, Header};
        match self {
            Header(e) => sub_pop(e.as_ref(), refs),
            Footer(e) => sub_pop(e.as_ref(), refs),
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::DecorationElement::{Footer, Header};
        vec![match self {
            Header(e) => sub_res(*e, refs).into(),
            Footer(e) => sub_res(*e, refs).into(),
        }]
    }
}

impl ResolvableRefs for c::SubTopic {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::SubTopic::{BodyElement, Title};
        match self {
            Title(e) => sub_pop(e.as_ref(), refs),
            BodyElement(e) => e.populate_targets(refs),
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::SubTopic::{BodyElement, Title};
        match self {
            Title(e) => vec![sub_res(*e, refs).into()],
            BodyElement(e) => e.resolve_refs(refs).drain(..).map(Into::into).collect(),
        }
    }
}

impl ResolvableRefs for c::SubSidebar {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::SubSidebar as S;
        match self {
            S::Topic(e) => sub_pop(e.as_ref(), refs),
            S::Title(e) => sub_pop(e.as_ref(), refs),
            S::Subtitle(e) => sub_pop(e.as_ref(), refs),
            S::BodyElement(e) => e.populate_targets(refs),
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::SubSidebar as S;
        vec![match self {
            S::Topic(e) => sub_res(*e, refs).into(),
            S::Title(e) => sub_res(*e, refs).into(),
            S::Subtitle(e) => sub_res(*e, refs).into(),
            S::BodyElement(e) => return e.resolve_refs(refs).drain(..).map(Into::into).collect(),
        }]
    }
}

impl ResolvableRefs for c::SubDLItem {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::SubDLItem::{Classifier, Definition, Term};
        match self {
            Term(e) => sub_pop(e.as_ref(), refs),
            Classifier(e) => sub_pop(e.as_ref(), refs),
            Definition(e) => sub_pop(e.as_ref(), refs),
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::SubDLItem::{Classifier, Definition, Term};
        vec![match self {
            Term(e) => sub_res(*e, refs).into(),
            Classifier(e) => sub_res(*e, refs).into(),
            Definition(e) => sub_res(*e, refs).into(),
        }]
    }
}

impl ResolvableRefs for c::SubField {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::SubField::{FieldBody, FieldName};
        match self {
            FieldName(e) => sub_pop(e.as_ref(), refs),
            FieldBody(e) => sub_pop(e.as_ref(), refs),
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::SubField::{FieldBody, FieldName};
        vec![match self {
            FieldName(e) => sub_res(*e, refs).into(),
            FieldBody(e) => sub_res(*e, refs).into(),
        }]
    }
}

impl ResolvableRefs for c::SubOptionListItem {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::SubOptionListItem::{Description, OptionGroup};
        match self {
            OptionGroup(e) => sub_sub_pop(e.as_ref(), refs),
            Description(e) => sub_pop(e.as_ref(), refs),
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::SubOptionListItem::{Description, OptionGroup};
        vec![match self {
            OptionGroup(e) => sub_sub_res(*e, refs).into(),
            Description(e) => sub_res(*e, refs).into(),
        }]
    }
}

impl ResolvableRefs for c::SubOption {
    fn populate_targets(&self, _: &mut TargetsCollected) {}
    fn resolve_refs(self, _: &TargetsCollected) -> Vec<Self> {
        vec![self]
    }
}

impl ResolvableRefs for c::SubLineBlock {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::SubLineBlock::{Line, LineBlock};
        match self {
            LineBlock(e) => sub_pop(e.as_ref(), refs),
            Line(e) => sub_pop(e.as_ref(), refs),
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::SubLineBlock::{Line, LineBlock};
        vec![match self {
            LineBlock(e) => sub_res(*e, refs).into(),
            Line(e) => sub_res(*e, refs).into(),
        }]
    }
}

impl ResolvableRefs for c::SubBlockQuote {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::SubBlockQuote::{Attribution, BodyElement};
        match self {
            Attribution(e) => sub_pop(e.as_ref(), refs),
            BodyElement(e) => e.populate_targets(refs),
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::SubBlockQuote::{Attribution, BodyElement};
        match self {
            Attribution(e) => vec![sub_res(*e, refs).into()],
            BodyElement(e) => e.resolve_refs(refs).drain(..).map(Into::into).collect(),
        }
    }
}

impl ResolvableRefs for c::SubFootnote {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::SubFootnote::{BodyElement, Label};
        match self {
            Label(e) => sub_pop(e.as_ref(), refs),
            BodyElement(e) => e.populate_targets(refs),
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::SubFootnote::{BodyElement, Label};
        match self {
            Label(e) => vec![sub_res(*e, refs).into()],
            BodyElement(e) => e.resolve_refs(refs).drain(..).map(Into::into).collect(),
        }
    }
}

impl ResolvableRefs for c::SubFigure {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::SubFigure::{BodyElement, Caption, Legend};
        match self {
            Caption(e) => sub_pop(e.as_ref(), refs),
            Legend(e) => sub_pop(e.as_ref(), refs),
            BodyElement(e) => e.populate_targets(refs),
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::SubFigure::{BodyElement, Caption, Legend};
        vec![match self {
            Caption(e) => sub_res(*e, refs).into(),
            Legend(e) => sub_res(*e, refs).into(),
            BodyElement(e) => return e.resolve_refs(refs).drain(..).map(Into::into).collect(),
        }]
    }
}

impl ResolvableRefs for c::SubTable {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::SubTable::{TableGroup, Title};
        match self {
            Title(e) => sub_pop(e.as_ref(), refs),
            TableGroup(e) => sub_pop(e.as_ref(), refs),
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::SubTable::{TableGroup, Title};
        vec![match self {
            Title(e) => sub_res(*e, refs).into(),
            TableGroup(e) => sub_res(*e, refs).into(),
        }]
    }
}

impl ResolvableRefs for c::SubTableGroup {
    fn populate_targets(&self, refs: &mut TargetsCollected) {
        use c::SubTableGroup::{TableBody, TableColspec, TableHead};
        match self {
            TableColspec(_) => {
                unimplemented!();
            }
            TableHead(e) => {
                for c in e.children() {
                    sub_sub_pop(c, refs);
                }
            }
            TableBody(e) => {
                for c in e.children() {
                    sub_sub_pop(c, refs);
                }
            }
        }
    }
    fn resolve_refs(self, refs: &TargetsCollected) -> Vec<Self> {
        use c::SubTableGroup::{TableBody, TableColspec, TableHead};
        vec![match self {
            TableColspec(e) => TableColspec(e),
            TableHead(mut e) => {
                let new: Vec<_> = e
                    .children_mut()
                    .drain(..)
                    .map(|c| sub_sub_res(c, refs))
                    .collect();
                e.children_mut().extend(new);
                TableHead(e)
            }
            TableBody(mut e) => {
                let new: Vec<_> = e
                    .children_mut()
                    .drain(..)
                    .map(|c| sub_sub_res(c, refs))
                    .collect();
                e.children_mut().extend(new);
                TableBody(e)
            }
        }]
    }
}
