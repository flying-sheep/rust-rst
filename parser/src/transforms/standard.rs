/*! Perform standard transforms.
 *
 * Hyperlinks
 * ----------
 *
 * See <https://docutils.sourceforge.net/docs/ref/rst/restructuredtext.html#hyperlink-targets>
 *
 * Links can have internal or external targets.
 * In the source, targets look like:
 *
 * ```restructuredtext
 * .. targetname1:
 * .. targetname2:
 *
 * some paragraph or list item or so
 * ```
 *
 * or:
 *
 * ```restructuredtext
 * .. targetname1:
 * .. targetname2: https://link
 * ```
 *
 * There’s also anonymous links and targets without names.
 *
 * TODO: continue documenting how it’s done via <https://repo.or.cz/docutils.git/blob/HEAD:/docutils/docutils/transforms/references.py>
 *
 * Footnotes
 * ---------
 *
 * See <https://docutils.sourceforge.io/docs/ref/rst/restructuredtext.html#footnotes>
 *
 * Footnotes can be numbered or symbolic.
 * In the source, they are split into two parts: footnote references and footnotes.
 *
 * Their order is defined by the order of the footnotes, not references.
 */

use std::{collections::HashMap, iter::once, num::NonZero, vec};

use document_tree::{
    Document, HasChildren, LabelledFootnote as _,
    attribute_types::{FootnoteType, ID, NameToken},
    element_categories as c,
    elements::{self as e, Element},
    extra_attributes::{ExtraAttributes, FootnoteTypeExt},
    url::Url,
};
use linearize::{Linearize, StaticMap};

use super::{Transform, Visit};

#[must_use]
pub fn standard_transform(doc: Document) -> Document {
    let mut pass1 = Pass1::default();
    let doc = pass1.transform(doc);
    let mut pass2 = Pass2::from(&pass1);
    pass2.visit(&doc);
    Pass3::from(&pass2).transform(doc)
}

#[derive(Debug)]
#[allow(dead_code)]
enum NamedTargetType {
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
        matches!(self, T::SectionTitle | T::Citation)
    }
}

const ONE: NonZero<usize> = NonZero::<usize>::MIN;

/// Pass 1: Number footnotes, and add IDs to footnote references and footnotes.
///
/// Needs to be separate pass, since resolving `refid`s for footnote references requires already-assigned footnote numbers.
/// Therefore, we do that here, then (in pass 2) resolve references, and finally (in pass 3) transform the footnotes.
#[derive(Default, Debug)]
struct Pass1 {
    /// Store numbers footnotes.
    /// Symbol ones can only be in order, so `_.values().sort() == 1..=_.len()`
    /// Number ones can have gaps due to explicitly numbered ones.
    footnotes: StaticMap<FootnoteType, HashMap<ID, NonZero<usize>>>,
    /// Numbers of auto-nubered footnotes in order of appearance.
    auto_numbered_footnotes: StaticMap<AutoNumberedFootnoteType, Vec<NonZero<usize>>>,
    /// Number of encountered anonymous footnotes. Only used for ID generation.
    n_anon_footnotes: usize,
    /// Number of encountered footnote references. Only used for ID generation.
    n_footnote_refs: usize,
}
impl Pass1 {
    /// Get next footnote number for a type.
    ///
    /// See <https://docutils.sourceforge.io/docs/ref/rst/restructuredtext.html#mixed-manual-and-auto-numbered-footnotes>
    fn next_footnote(&mut self, typ: FootnoteType) -> NonZero<usize> {
        let footnotes = &mut self.footnotes[typ];
        match typ {
            FootnoteType::Number => {
                let Some(n) = NonZero::new(footnotes.len()) else {
                    return ONE;
                };
                let mut ordered: Vec<_> = footnotes.values().copied().collect();
                ordered.sort_unstable();
                ordered
                    .iter()
                    .copied()
                    .zip(1usize..) // https://github.com/rust-lang/rust/pull/127534
                    .enumerate()
                    .find_map(|(i, (n1, n2))| (n1.get() != n2).then_some(ONE.saturating_add(i)))
                    .unwrap_or(n)
            }
            FootnoteType::Symbol => {
                if cfg!(debug_assertions) {
                    let mut vals: Vec<usize> =
                        footnotes.values().copied().map(Into::into).collect();
                    vals.sort_unstable();
                    assert_eq!(vals, (1..=footnotes.len()).collect::<Vec<_>>());
                }
                ONE.saturating_add(footnotes.len())
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Linearize)]
enum AutoNumberedFootnoteType {
    Anomymous,
    Named,
}

impl AutoNumberedFootnoteType {
    fn from_names(names: &[NameToken]) -> Self {
        if names.is_empty() {
            Self::Anomymous
        } else {
            Self::Named
        }
    }
}

impl Transform for Pass1 {
    /// Add (auto-)id and running count to “ids” of footnotes
    fn transform_footnote(&mut self, mut e: e::Footnote) -> impl Iterator<Item = c::BodyElement> {
        // Get next or stored footnote number
        let n = match e
            .extra()
            .auto
            .map(|t| self.next_footnote(t))
            .ok_or(())
            .or_else::<anyhow::Error, _>(|()| Ok(e.get_label()?.parse()?))
        {
            Ok(n) => n,
            Err(err) => {
                let t = e::Problematic::with_children(vec![err.to_string().into()]).into();
                return once(e::Paragraph::with_children(vec![t]).into());
            }
        };

        // Get ID from name or create one from the running count
        let id = if let Some(name) = e.names().first() {
            name.0.as_str().into()
        } else {
            self.n_anon_footnotes += 1;
            ID(format!("footnote-{}", self.n_anon_footnotes))
        };
        e.ids_mut().push(id.clone());

        // Add footnote to the correct mapping
        self.footnotes[e.footnote_type()].insert(id.clone(), n);

        // Keep track of named vs anonymous footnotes for auto-numbering refs later
        if matches!(e.extra().auto, Some(FootnoteType::Number)) {
            let t = AutoNumberedFootnoteType::from_names(e.names());
            self.auto_numbered_footnotes[t].push(n);
        }

        // Standard transform
        self.transform_children(&mut e, Self::transform_sub_footnote);
        once(e.into())
    }
    /// Give each reference an ID. We don’t need to do more.
    fn transform_footnote_reference(
        &mut self,
        mut e: e::FootnoteReference,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        // Add running count ID
        self.n_footnote_refs += 1;
        e.ids_mut()
            .push(ID(format!("footnote-reference-{}", self.n_footnote_refs)));

        // Standard transform
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Substitution {
    content: Vec<c::TextOrInlineElement>,
    /// If true and the sibling before the reference is a text node,
    /// the text node gets right-trimmed.
    ltrim: bool,
    /// Same as `ltrim` with the sibling after the reference.
    rtrim: bool,
}

#[derive(Debug)]
struct Pass2<'p1> {
    pass1: &'p1 Pass1,
    named_targets: HashMap<NameToken, NamedTargetType>,
    substitutions: HashMap<NameToken, Substitution>,
    normalized_substitutions: HashMap<String, Substitution>,
    /// Footnote references.
    /// Multiple numbered ones can point to the same number.
    footnote_refs: StaticMap<FootnoteType, HashMap<ID, NonZero<usize>>>,
    /// Number of symbol footnote references.
    n_symbol_footnote_refs: usize,
    /// Number of auto-numbered footnote references.
    n_auto_num_footnote_refs: StaticMap<AutoNumberedFootnoteType, usize>,
}
impl<'p1> From<&'p1 Pass1> for Pass2<'p1> {
    fn from(pass1: &'p1 Pass1) -> Self {
        Self {
            pass1,
            named_targets: HashMap::new(),
            substitutions: HashMap::new(),
            normalized_substitutions: HashMap::new(),
            footnote_refs: StaticMap::default(),
            n_symbol_footnote_refs: 0,
            n_auto_num_footnote_refs: StaticMap::default(),
        }
    }
}

/// Pass 2.
///
/// - Populate substitution definitions.
/// - Populate (link) targets.
/// - Resolve which footnotes are referenced by footnote references.
impl<'tree> Visit<'tree> for Pass2<'_> {
    fn visit_substitution_definition(&mut self, e: &'tree e::SubstitutionDefinition) {
        let subst = Substitution {
            content: e.children().clone(),
            ltrim: e.extra().ltrim,
            rtrim: e.extra().rtrim,
        };
        for name in e.names() {
            if self.substitutions.contains_key(name) {
                // TODO: Duplicate substitution name (level 3 system message).
            }
            // Intentionally overriding any previous values.
            self.substitutions.insert(name.clone(), subst.clone());
            self.normalized_substitutions
                .insert(name.0.to_lowercase(), subst.clone());
        }
    }
    fn visit_target(&mut self, e: &'tree e::Target) {
        if let Some(uri) = &e.extra().refuri {
            for name in e.names() {
                self.named_targets
                    .insert(name.clone(), NamedTargetType::ExternalLink(uri.clone()));
            }
        }
        // TODO: as is, people can only refer to the target directly containing the URL.
        // add refid and refnames to some HashMap and follow those later.
    }
    fn visit_footnote_reference(&mut self, e: &'tree e::FootnoteReference) {
        let id = e.ids().first().unwrap();
        let n = match e.extra().auto {
            Some(FootnoteType::Symbol) => {
                self.n_symbol_footnote_refs += 1;
                NonZero::new(self.n_symbol_footnote_refs).unwrap()
            }
            Some(FootnoteType::Number) => {
                let t = AutoNumberedFootnoteType::from_names(e.names());
                self.n_auto_num_footnote_refs[t] += 1;
                self.pass1.auto_numbered_footnotes[t][self.n_auto_num_footnote_refs[t] - 1]
            }
            None => e.get_label().unwrap().parse().unwrap(),
        };

        self.footnote_refs[e.footnote_type()].insert(id.clone(), n);

        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
}

#[derive(Debug)]
struct Pass3<'p2>(&'p2 Pass2<'p2>);
impl<'p2> Pass3<'p2> {
    fn target_url<'t>(self: &'t Pass3<'p2>, refname: &[NameToken]) -> Option<&'t Url> {
        // TODO: Check if the target would expand circularly
        assert!(
            refname.len() == 1,
            "Expected exactly one name in a reference."
        );
        let name = refname[0].clone();
        match self.0.named_targets.get(&name)? {
            NamedTargetType::ExternalLink(url) => Some(url),
            _ => unimplemented!(),
        }
    }

    fn substitution<'t>(self: &'t Pass3<'p2>, refname: &[NameToken]) -> Option<&'t Substitution> {
        // TODO: Check if the substitution would expand circularly
        assert!(
            refname.len() == 1,
            "Expected exactly one name in a substitution reference."
        );
        let name = refname[0].clone();
        self.0
            .substitutions
            .get(&name)
            .or_else(|| self.0.normalized_substitutions.get(&name.0.to_lowercase()))
    }
}

impl<'p2> From<&'p2 Pass2<'p2>> for Pass3<'p2> {
    fn from(p: &'p2 Pass2<'p2>) -> Self {
        Pass3(p)
    }
}

/// 3rd pass.
impl Transform for Pass3<'_> {
    fn transform_substitution_definition(
        &mut self,
        _: e::SubstitutionDefinition,
    ) -> impl Iterator<Item = c::BodyElement> {
        None.into_iter()
    }
    fn transform_substitution_reference(
        &mut self,
        e: e::SubstitutionReference,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        let r: Box<dyn Iterator<Item = c::TextOrInlineElement>> = if let Some(Substitution {
            content,
            ltrim,
            rtrim,
        }) =
            self.substitution(&e.extra().refname)
        {
            // (level 3 system message).
            // TODO: ltrim and rtrim.
            if *ltrim || *rtrim {
                dbg!(content, ltrim, rtrim);
            }
            Box::new(content.clone().into_iter())
        } else {
            // Undefined substitution name (level 3 system message).
            // TODO: This replaces the reference by a Problematic node.
            // The corresponding SystemMessage node should go in a generated
            // section with class "system-messages" at the end of the document.
            let mut replacement: Box<e::Problematic> = Box::default();
            replacement
                .children_mut()
                .push(c::TextOrInlineElement::String(Box::new(format!(
                    "|{}|",
                    e.extra().refname[0].0
                ))));
            // TODO: Create an ID for replacement for the system_message to reference.
            // TODO: replacement.refid pointing to the system_message.

            Box::new(once(c::TextOrInlineElement::Problematic(replacement)))
        };
        r
    }
    fn transform_reference(
        &mut self,
        mut e: e::Reference,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        if e.extra().refuri.is_none()
            && let Some(uri) = self.target_url(&e.extra().refname)
        {
            e.extra_mut().refuri = Some(uri.clone());
        }
        once(e.into())
    }
    fn transform_footnote(&mut self, mut e: e::Footnote) -> impl Iterator<Item = c::BodyElement> {
        /* TODO: https://docutils.sourceforge.io/docs/ref/doctree.html#footnote-reference
        1. see above
        2. (in resolve_refs) set `footnote_reference[refid]`s, `footnote[backref]`s and `footnote>label`
        */
        let id = e.ids().first().unwrap();
        let num = self.0.pass1.footnotes[e.footnote_type()].get(id).unwrap();
        if e.get_label().is_err() {
            e.children_mut().insert(
                0,
                e::Label::with_children(vec![num.to_string().into()]).into(),
            );
        }

        // backrefs
        e.extra_mut().backrefs = self.0.footnote_refs[e.footnote_type()]
            .iter()
            .filter(|&(_, num2)| num == num2)
            .map(|(refid, _)| refid.clone())
            .collect();

        // standard transform
        self.transform_children(&mut e, Self::transform_sub_footnote);
        once(e.into())
    }
    fn transform_footnote_reference(
        &mut self,
        mut e: e::FootnoteReference,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        // TODO: dedupe
        // https://docutils.sourceforge.io/docs/ref/doctree.html#footnote-reference
        let refid = e.ids().first().unwrap();
        let n = self.0.footnote_refs[e.footnote_type()].get(refid).unwrap();

        // get referenced footnote ID
        e.extra_mut().refid = self.0.pass1.footnotes[e.footnote_type()]
            .iter()
            .find_map(|(k, v)| (v == n).then_some(k.clone()));

        // add label
        if e.get_label().is_err() {
            e.children_mut().insert(0, n.to_string().into());
        }

        // standard transform
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
}
