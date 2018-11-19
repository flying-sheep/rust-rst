pub mod token;
pub mod serialize;
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

use failure::Error;
use failure_derive::Fail;

use pest::Parser;

use crate::document_tree::{
    HasChildren,
    elements::{
        Document,
        Title,
    },
    element_categories::{
        StructuralSubElement
    },
};


#[derive(Debug, Fail)]
enum ConversionError {
    #[fail(display = "unknown rule: {:?}", rule)]
    UnknownRuleError {
        rule: Rule,
    },
}


fn convert_ssubel(pair: pest::iterators::Pair<Rule>) -> Result<StructuralSubElement, Error> {
    match pair.as_rule() {
        Rule::title => Ok(convert_title(pair).into()),
        rule => Err(ConversionError::UnknownRuleError { rule }.into()),
    }
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


/// tokens to Document tree. resolves sections, but not references
pub fn parse(source: &str) -> Result<Document, Error> {
    let pairs = pest_rst::RstParser::parse(pest_rst::Rule::document, source)?;
    let structural_elems = pairs.map(convert_ssubel).collect::<Result<_, _>>()?;
    Ok(Document::with_children(structural_elems))
}


/// only until we can serialize DocumentTrees
pub fn serialize_json<W>(source: &str, stream: W) -> Result<(), Error> where W: Write {
    use self::pest_rst::{RstParser, Rule};
    use self::serialize::PairsWrap;
    
    let parsed = RstParser::parse(Rule::document, source)?;
    serde_json::to_writer(stream, &PairsWrap(parsed))?;
    Ok(())
}
