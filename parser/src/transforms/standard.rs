/*! Perform standard transforms.
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
*/

use std::{collections::HashMap, iter::once, num::NonZero};

use document_tree::{
    Document, HasChildren, LabelledFootnote as _,
    attribute_types::{AutoFootnoteType, ID, NameToken},
    element_categories as c,
    elements::{self as e, Element},
    extra_attributes::ExtraAttributes,
    url::Url,
};

use super::{Transform, Visit};

#[must_use]
pub fn standard_transform(doc: Document) -> Document {
    let mut pass1 = Pass1::default();
    let doc = pass1.transform(doc);
    let mut pass2 = Pass2::from(pass1);
    pass2.visit(&doc);
    Pass3::from(pass2).transform(doc)
}

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

const ONE: NonZero<usize> = NonZero::<usize>::MIN;

/// Pass 1: Assign IDs to footnotes and footnote references.
///
/// Needs to be separate pass, since resolving `refid`s for footnote references requires already-assigned footnote IDs.
/// Therefore, we do that here, then resolve references in the visit part of the second pass, and finally transform the footnotes.
#[derive(Default, Debug)]
struct Pass1 {
    /// Numbered footnotes can have gaps due to explicitly numbered ones.
    footnotes_number: HashMap<ID, NonZero<usize>>,
    /// Symbol footnotes can only be in order, so `_.values().sort() == 1..=_.len()`
    footnotes_symbol: HashMap<ID, NonZero<usize>>,
    /// Number of encountered footnotes. Usually `footnotes_number.len()+footnotes_symbol.len()`.
    n_footnotes: usize,
    /// Number of encountered footnote references.
    n_footnote_refs: usize,
}
impl Pass1 {
    /// Get next footnote number for a type.
    ///
    /// See <https://docutils.sourceforge.io/docs/ref/rst/restructuredtext.html#mixed-manual-and-auto-numbered-footnotes>
    fn next_footnote(&mut self, typ: AutoFootnoteType) -> NonZero<usize> {
        match typ {
            AutoFootnoteType::Number => self
                .footnotes_number
                .values()
                .copied()
                .min()
                .map_or(ONE, |n| n.saturating_add(1)),
            AutoFootnoteType::Symbol => {
                if cfg!(debug_assertions) {
                    let mut vals: Vec<usize> = self
                        .footnotes_symbol
                        .values()
                        .copied()
                        .map(Into::into)
                        .collect();
                    vals.sort();
                    assert_eq!(vals, (1..=self.footnotes_symbol.len()).collect::<Vec<_>>());
                }
                ONE.saturating_add(self.footnotes_symbol.len())
            }
        }
    }
}

impl Transform for Pass1 {
    /// Add auto-id and running count to “ids” of footnotes
    fn transform_footnote(&mut self, mut e: e::Footnote) -> impl Iterator<Item = c::BodyElement> {
        // Get next or stored footnote number
        let n = e
            .extra()
            .auto
            .map(|t| Some(self.next_footnote(t)))
            .unwrap_or_else(|| e.get_label().ok().and_then(|l| l.parse().ok()));
        // If we got one, add it to the correct mapping
        if let Some(n) = n {
            for id in e.ids() {
                match e.extra().auto {
                    Some(AutoFootnoteType::Symbol) => self.footnotes_symbol.insert(id.clone(), n),
                    _ => self.footnotes_number.insert(id.clone(), n),
                };
            }
        }

        // Add running count ID
        self.n_footnotes += 1;
        e.ids_mut()
            .push(ID(format!("footnote-{}", self.n_footnotes)));

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

#[derive(Clone, Debug)]
struct Substitution {
    content: Vec<c::TextOrInlineElement>,
    /// If true and the sibling before the reference is a text node,
    /// the text node gets right-trimmed.
    ltrim: bool,
    /// Same as `ltrim` with the sibling after the reference.
    rtrim: bool,
}

#[derive(Debug)]
struct Pass2 {
    pass1: Pass1,
    named_targets: HashMap<NameToken, NamedTargetType>,
    substitutions: HashMap<NameToken, Substitution>,
    normalized_substitutions: HashMap<String, Substitution>,
}
impl From<Pass1> for Pass2 {
    fn from(pass1: Pass1) -> Self {
        Self {
            pass1,
            named_targets: HashMap::new(),
            substitutions: HashMap::new(),
            normalized_substitutions: HashMap::new(),
        }
    }
}

/// 2nd pass
impl<'tree> Visit<'tree> for Pass2 {
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
    fn visit_footnote(&mut self, e: &'tree e::Footnote) {
        /* TODO: https://docutils.sourceforge.io/docs/ref/doctree.html#footnote-reference
        1. (here) add auto-id and running count to “ids” of footnote references and footnotes
        2. see below
        */
        let n = match e.extra().auto {
            Some(t @ AutoFootnoteType::Number) => {
                let n = self.next_footnote(t, !e.names().is_empty());
                for name in e.names() {
                    self.named_targets
                        .insert(name.clone(), NamedTargetType::LabeledFootnote(n));
                }
                Some(n)
            }
            Some(t @ AutoFootnoteType::Symbol) => Some(self.next_footnote(t, false)),
            None => e.get_label().ok().and_then(|l| l.parse().ok()),
        };
        if let Some(n) = n {
            for id in e.ids() {
                match e.extra().auto {
                    Some(AutoFootnoteType::Symbol) => self.footnotes_symbol.insert(id.clone(), n),
                    _ => self.footnotes_number.insert(id.clone(), n),
                };
            }
        }
        for c in e.children() {
            self.visit_sub_footnote(c);
        }
    }
    fn visit_footnote_reference(&mut self, e: &'tree e::FootnoteReference) {
        // TODO: dedupe
        let n = match e.extra().auto {
            Some(t @ AutoFootnoteType::Number) => {
                let n = self.next_footnote(t, true, !e.names().is_empty());
                for name in e.names() {
                    self.named_targets
                        .insert(name.clone(), NamedTargetType::LabeledFootnote(n));
                }
                Some(n)
            }
            Some(t @ AutoFootnoteType::Symbol) => Some(self.next_footnote(t, true, false)),
            None => e.get_label().ok().and_then(|l| l.parse().ok()),
        };
        if let Some(n) = n {
            for id in e.ids() {
                match e.extra().auto {
                    Some(AutoFootnoteType::Symbol) => {
                        self.footnote_refs_symbol.insert(id.clone(), n)
                    }
                    _ => self.footnote_refs_number.insert(id.clone(), n),
                };
            }
        }
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
}

#[derive(Debug)]
struct Pass3(Pass2);
impl Pass3 {
    fn target_url<'t>(self: &'t Pass3, refname: &[NameToken]) -> Option<&'t Url> {
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

    fn substitution<'t>(self: &'t Pass3, refname: &[NameToken]) -> Option<&'t Substitution> {
        // TODO: Check if the substitution would expand circularly
        assert!(
            refname.len() == 1,
            "Expected exactly one name in a substitution reference."
        );
        let name = refname[0].clone();
        self.0
            .substitutions
            .get(&name)
            .or_else(|| self.normalized_substitutions.get(&name.0.to_lowercase()))
    }
}

impl From<Pass2> for Pass3 {
    fn from(p: Pass2) -> Self {
        Pass3(p)
    }
}

/// 3rd pass.
impl Transform for Pass3 {
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
        if e.extra().refuri.is_none() {
            if let Some(uri) = self.target_url(&e.extra().refname) {
                e.extra_mut().refuri = Some(uri.clone());
            }
        }
        once(e.into())
    }
    fn transform_footnote(&mut self, mut e: e::Footnote) -> impl Iterator<Item = c::BodyElement> {
        /* TODO: https://docutils.sourceforge.io/docs/ref/doctree.html#footnote-reference
        1. see above
        2. (in resolve_refs) set `footnote_reference[refid]`s, `footnote[backref]`s and `footnote>label`
        */
        if e.get_label().is_err() {
            let label = e
                .ids()
                .iter()
                .find_map(|id| match e.extra().auto {
                    Some(AutoFootnoteType::Symbol) => self.footnotes_symbol.get(id),
                    _ => self.footnotes_number.get(id),
                })
                .map_or_else(|| "???".into(), ToString::to_string);
            e.children_mut()
                .insert(0, e::Label::with_children(vec![label.into()]).into());
        }
        self.transform_children(&mut e, Self::transform_sub_footnote);
        once(e.into())
    }
    fn transform_footnote_reference(
        &mut self,
        mut e: e::FootnoteReference,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        // TODO: dedupe
        // https://docutils.sourceforge.io/docs/ref/doctree.html#footnote-reference
        let n = e
            .ids()
            .iter()
            .find_map(|id| match e.extra().auto {
                Some(AutoFootnoteType::Symbol) => self.footnote_refs_symbol.get(id),
                _ => self.footnote_refs_number.get(id),
            })
            .expect("Footnote reference without id");
        e.extra_mut().refid = Some(ID(format!("footnote-{n}")));
        if e.get_label().is_err() {
            e.children_mut().insert(0, n.to_string().into());
        }
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
}
