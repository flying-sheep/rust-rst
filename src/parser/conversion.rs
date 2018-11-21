use url::Url;
use failure::Error;
use failure_derive::Fail;
use pest::iterators::{Pairs,Pair};

use crate::document_tree::{
    HasChildren,
    elements as e,
    element_categories as c,
    attribute_types::ID,
    extra_attributes,
};

use super::pest_rst::Rule;

#[derive(Debug, Fail)]
enum ConversionError {
    #[fail(display = "unknown rule: {:?}", rule)]
    UnknownRuleError {
        rule: Rule,
    },
}


pub fn convert_document(pairs: Pairs<Rule>) -> Result<e::Document, Error> {
    let structural_elems = pairs.map(convert_ssubel).collect::<Result<_,_>>()?;
    Ok(e::Document::with_children(structural_elems))
}


fn convert_ssubel(pair: Pair<Rule>) -> Result<c::StructuralSubElement, Error> {
    // TODO: This is just a proof of concep. Keep closely to DTD in final version!
    match pair.as_rule() {
        Rule::title => Ok(convert_title(pair).into()),
        Rule::paragraph => Ok(to_ssub(e::Paragraph::with_children(vec![pair.as_str().into()]))),
        Rule::target => Ok(to_ssub(convert_target(pair)?)),
        Rule::admonition_gen => Ok(to_ssub(convert_admonition_gen(pair)?)),
        rule => Err(ConversionError::UnknownRuleError { rule }.into()),
    }
}


fn to_ssub<E>(elem: E) -> c::StructuralSubElement where E: Into<c::BodyElement> {
    let belm: c::BodyElement = elem.into();
    let sstruc: c::SubStructure = belm.into();
    sstruc.into()
}


fn convert_title(pair: Pair<Rule>) -> e::Title {
    let mut title: Option<&str> = None;
    let mut _adornment_char: Option<char> = None;
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::line => title = Some(p.as_str()),
            Rule::adornments => _adornment_char = Some(p.as_str().chars().next().expect("Empty adornment?")),
            rule => panic!("Unexpected rule in title: {:?}", rule),
        };
    }
    // TODO adornment char
    e::Title::with_children(vec![
        title.expect("No text in title").into()
    ])
}

fn convert_target(pair: Pair<Rule>) -> Result<e::Target, Error> {
    let mut attrs = extra_attributes::Target {
        anonymous: false,
        ..Default::default()
    };
    for p in pair.into_inner() {
        match p.as_rule() {
            // TODO: or is it refnames?
            Rule::target_name_uq | Rule::target_name_qu => attrs.refid = Some(ID(p.as_str().to_owned())),
            Rule::link_target => attrs.refuri = Some(Url::parse(p.as_str())?),
            rule => panic!("Unexpected rule in target: {:?}", rule),
        }
    }
    Ok(e::Target::new(Default::default(), attrs))
}

fn convert_admonition_gen(pair: Pair<Rule>) -> Result<c::BodyElement, Error> {
    let mut iter = pair.into_inner();
    let typ = iter.next().unwrap().as_str();
    // TODO: in reality it contains body elements.
    let children: Vec<c::BodyElement> = iter.map(|p| e::Paragraph::with_children(vec![p.as_str().into()]).into()).collect();
    Ok(match typ {
        "attention" => e::Attention::with_children(children).into(),
        "hint"      =>      e::Hint::with_children(children).into(),
        "note"      =>      e::Note::with_children(children).into(),
        "caution"   =>   e::Caution::with_children(children).into(),
        "danger"    =>    e::Danger::with_children(children).into(),
        "error"     =>     e::Error::with_children(children).into(),
        "important" => e::Important::with_children(children).into(),
        "tip"       =>       e::Tip::with_children(children).into(),
        "warning"   =>   e::Warning::with_children(children).into(),
        typ         => panic!("Unknown admontion type {}!", typ),
    })
}
