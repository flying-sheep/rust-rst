use std::{collections::HashMap, iter::once, num::NonZero};

use document_tree::{
    HasChildren,
    attribute_types::{AutoFootnoteType, ID, NameToken},
    element_categories as c,
    elements::{self as e, Element},
    extra_attributes::ExtraAttributes,
    url::Url,
};

use super::{Visit, VisitMut};
use crate::transform_children;

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
pub(super) struct TargetCollector {
    named_targets: HashMap<NameToken, NamedTargetType>,
    substitutions: HashMap<NameToken, Substitution>,
    normalized_substitutions: HashMap<String, Substitution>,
    /// Holds used footnote numbers.
    footnotes_number: HashMap<ID, NonZero<usize>>,
    footnotes_symbol: HashMap<ID, NonZero<usize>>,
}
impl TargetCollector {
    fn target_url<'t>(self: &'t TargetCollector, refname: &[NameToken]) -> Option<&'t Url> {
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
        self: &'t TargetCollector,
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

    fn next_footnote(&mut self, typ: AutoFootnoteType, named: bool) -> NonZero<usize> {
        // Auto-numbered named footnotes get the *lowest* available number.
        // Auto-numbered anonymous footnotes get the *next* available number.
        // See <https://docutils.sourceforge.io/docs/ref/rst/restructuredtext.html#mixed-manual-and-auto-numbered-footnotes>
        let it = match typ {
            AutoFootnoteType::Number => &mut self.footnotes_number,
            AutoFootnoteType::Symbol => &mut self.footnotes_symbol,
        }
        .values()
        .copied();
        if named { it.min() } else { it.max() }
            .map_or(NonZero::new(1usize).unwrap(), |n| n.saturating_add(1))
    }
}

/// First pass
impl<'tree> Visit<'tree> for TargetCollector {
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
}

// Second pass
impl VisitMut for TargetCollector {
    fn visit_substitution_definition_mut(
        &mut self,
        _: e::SubstitutionDefinition,
    ) -> impl Iterator<Item = c::BodyElement> {
        None.into_iter()
    }
    fn visit_footnote_mut(&mut self, mut e: e::Footnote) -> impl Iterator<Item = c::BodyElement> {
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
        transform_children!(e, self.visit_sub_footnote_mut);
        once(e.into())
    }
    fn visit_reference_mut(
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
    fn visit_substitution_reference_mut(
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
}
