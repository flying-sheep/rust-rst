use anyhow::Error;
use pest::iterators::Pair;

use document_tree::{
    CommonAttributes, Element, ExtraAttributes, HasChildren, attribute_types as at,
    element_categories as c, elements as e,
    extra_attributes::{self as a, FootnoteType},
    url::Url,
};

use super::whitespace_normalize_name;
use crate::pest_rst::Rule;

pub fn convert_inline(pair: Pair<Rule>) -> Result<c::TextOrInlineElement, Error> {
    Ok(match pair.as_rule() {
        Rule::str | Rule::str_nested => pair.as_str().into(),
        Rule::escaped_char => pair.as_str()[1..].into(),
        Rule::ws_newline => " ".to_owned().into(),
        Rule::reference => convert_reference(pair)?,
        Rule::substitution_name => convert_substitution_ref(&pair).into(),
        Rule::emph => e::Emphasis::with_children(convert_inlines(pair)?).into(),
        Rule::strong => e::Strong::with_children(convert_inlines(pair)?).into(),
        Rule::literal => e::Literal::with_children(vec![pair.as_str().to_owned()]).into(),
        Rule::footnote_reference => convert_footnote_reference(pair).into(),
        rule => unimplemented!("unknown rule {:?}", rule),
    })
}

pub fn convert_inlines(pair: Pair<Rule>) -> Result<Vec<c::TextOrInlineElement>, Error> {
    pair.into_inner().map(convert_inline).collect()
}

fn convert_reference(pair: Pair<Rule>) -> Result<c::TextOrInlineElement, Error> {
    let concrete = pair.into_inner().next().unwrap();
    match concrete.as_rule() {
        Rule::reference_target => convert_reference_target(concrete).map(Into::into),
        Rule::reference_explicit => unimplemented!("explicit reference"),
        Rule::reference_auto => Ok(convert_reference_auto(concrete)),
        _ => unreachable!(),
    }
}

fn convert_reference_target(concrete: Pair<'_, Rule>) -> Result<e::Reference, Error> {
    let rt_inner = concrete.into_inner().next().unwrap();
    Ok(match rt_inner.as_rule() {
        Rule::reference_target_uq => e::Reference::new(
            CommonAttributes::default(),
            a::Reference {
                name: Some(rt_inner.as_str().into()),
                refuri: None,
                refid: None,
                refname: vec![rt_inner.as_str().into()],
            },
            vec![rt_inner.as_str().into()],
        ),
        Rule::reference_target_qu => {
            let (text, reference) = {
                let mut text = None;
                let mut reference = None;
                for inner in rt_inner.clone().into_inner() {
                    match inner.as_rule() {
                        Rule::reference_text => text = Some(inner),
                        Rule::reference_bracketed => reference = Some(inner),
                        _ => unreachable!(),
                    }
                }
                (text, reference)
            };
            let trimmed_text = match (&text, &reference) {
                (Some(text), None) => text.as_str(),
                (_, Some(reference)) => text
                    .map(|text| text.as_str().trim_end_matches(|ch| " \n\r".contains(ch)))
                    .filter(|text| !text.is_empty())
                    .unwrap_or_else(|| reference.clone().into_inner().next().unwrap().as_str()),
                (None, None) => unreachable!(),
            };
            let (refuri, refname): (Option<Url>, Vec<at::NameToken>) =
                if let Some(reference) = reference {
                    let inner = reference.into_inner().next().unwrap();
                    match inner.as_rule() {
                        // The URL rules in our parser accept a narrow superset of
                        // valid URLs, so we need to handle false positives.
                        Rule::url => {
                            if let Ok(target) = Url::parse_absolute(inner.as_str()) {
                                (Some(target), Vec::new())
                            } else if inner.as_str().ends_with('_') {
                                // like target_name_qu (minus the final underscore)
                                let full_str = inner.as_str();
                                (None, vec![full_str[0..full_str.len() - 1].into()])
                            } else {
                                // like relative_reference
                                (Some(Url::parse_relative(inner.as_str())?), Vec::new())
                            }
                        }
                        Rule::target_name_qu => (None, vec![inner.as_str().into()]),
                        Rule::relative_reference => {
                            (Some(Url::parse_relative(inner.as_str())?), Vec::new())
                        }
                        _ => unreachable!(),
                    }
                } else {
                    (None, vec![trimmed_text.into()])
                };
            e::Reference::new(
                CommonAttributes::default(),
                a::Reference {
                    name: Some(trimmed_text.into()),
                    refuri,
                    refid: None,
                    refname,
                },
                vec![trimmed_text.into()],
            )
        }
        _ => unreachable!(),
    })
}

fn convert_reference_auto(concrete: Pair<'_, Rule>) -> c::TextOrInlineElement {
    let rt_inner = concrete.into_inner().next().unwrap();
    let str: c::TextOrInlineElement = rt_inner.as_str().into();
    let Ok(target) = (match rt_inner.as_rule() {
        Rule::url_auto => Url::parse_absolute(rt_inner.as_str()),
        Rule::email => Url::parse_absolute(&format!("mailto:{}", rt_inner.as_str())),
        _ => unreachable!(),
    }) else {
        // if our parser got a URL wrong, return it as a string
        return str;
    };
    e::Reference::new(
        CommonAttributes::default(),
        a::Reference {
            name: None,
            refuri: Some(target),
            refid: None,
            refname: Vec::new(),
        },
        vec![str],
    )
    .into()
}

fn convert_substitution_ref(pair: &Pair<Rule>) -> e::SubstitutionReference {
    let name = whitespace_normalize_name(pair.as_str());
    a::ExtraAttributes::with_extra(a::SubstitutionReference {
        refname: vec![at::NameToken(name)],
    })
}

fn convert_footnote_reference(pair: Pair<Rule>) -> e::FootnoteReference {
    let label = pair.into_inner().next().unwrap().as_str();

    let mut fr = e::FootnoteReference::default();
    if label.len() > 1 {
        let name = whitespace_normalize_name(&label[1..]);
        fr.names_mut().push(at::NameToken(name));
    }
    fr.extra_mut().auto = label.chars().next().unwrap().try_into().ok();
    if !fr.is_auto() {
        fr.children_mut().push(label.into());
    }
    fr
}
