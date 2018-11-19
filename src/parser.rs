pub mod token;
#[cfg(test)]
pub mod tests;

mod pest_rst {
    use pest_derive::Parser;
    
    #[derive(Parser)]
    #[grammar = "rst.pest"]
    pub struct RstParser;
}
use self::pest_rst::Rule;

use std::io::Write;

use url::Url;
use failure::Error;
use failure_derive::Fail;
use pest::Parser;

use crate::document_tree::{
    HasChildren,
    elements::{
        Document,
        Title,
        Paragraph,
        Target,
        Attention, Hint, Note, Caution, Danger, Error as ErrorEl, Important, Tip, Warning
    },
    element_categories::{
        StructuralSubElement,
        SubStructure,
        BodyElement,
    },
    attribute_types::ID,
    extra_attributes,
};


#[derive(Debug, Fail)]
enum ConversionError {
    #[fail(display = "unknown rule: {:?}", rule)]
    UnknownRuleError {
        rule: Rule,
    },
}


fn convert_ssubel(pair: pest::iterators::Pair<Rule>) -> Result<StructuralSubElement, Error> {
    // TODO: This is just a proof of concep. Keep closely to DTD in final version!
    match pair.as_rule() {
        Rule::title => Ok(convert_title(pair).into()),
        Rule::paragraph => Ok(to_ssub(Paragraph::with_children(vec![pair.as_str().into()]))),
        Rule::target => Ok(to_ssub(convert_target(pair)?)),
        Rule::admonition_gen => Ok(to_ssub(convert_admonition_gen(pair)?)),
        rule => Err(ConversionError::UnknownRuleError { rule }.into()),
    }
}


fn to_ssub<E>(elem: E) -> StructuralSubElement where E: Into<BodyElement> {
    let belm: BodyElement = elem.into();
    let sstruc: SubStructure = belm.into();
    sstruc.into()
}


fn convert_title(pair: pest::iterators::Pair<pest_rst::Rule>) -> Title {
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
    Title::with_children(vec![
        title.expect("No text in title").into()
    ])
}

fn convert_target(pair: pest::iterators::Pair<pest_rst::Rule>) -> Result<Target, Error> {
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
    Ok(Target::new(Default::default(), attrs))
}

fn convert_admonition_gen(pair: pest::iterators::Pair<pest_rst::Rule>) -> Result<BodyElement, Error> {
    let mut iter = pair.into_inner();
    let typ = iter.next().unwrap().as_str();
    // TODO: in reality it contains body elements.
    let children: Vec<BodyElement> = iter.map(|p| Paragraph::with_children(vec![p.as_str().into()]).into()).collect();
    Ok(match typ {
        "attention" => Attention::with_children(children).into(),
        "hint"      =>      Hint::with_children(children).into(),
        "note"      =>      Note::with_children(children).into(),
        "caution"   =>   Caution::with_children(children).into(),
        "danger"    =>    Danger::with_children(children).into(),
        "error"     =>   ErrorEl::with_children(children).into(),
        "important" => Important::with_children(children).into(),
        "tip"       =>       Tip::with_children(children).into(),
        "warning"   =>   Warning::with_children(children).into(),
        typ         => panic!("Unknown admontion type {}!", typ),
    })
}


/// tokens to Document tree. resolves sections, but not references
pub fn parse(source: &str) -> Result<Document, Error> {
    let pairs = pest_rst::RstParser::parse(pest_rst::Rule::document, source)?;
    let structural_elems = pairs.map(convert_ssubel).collect::<Result<_, _>>()?;
    Ok(Document::with_children(structural_elems))
}


/// only until we can serialize DocumentTrees
pub fn serialize_json<W>(source: &str, stream: W) -> Result<(), Error> where W: Write {
    let parsed = parse(source)?;
    serde_json::to_writer(stream, &parsed)?;
    Ok(())
}
