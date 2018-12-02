mod block;

use failure::Error;
use failure_derive::Fail;
use pest::iterators::Pairs;

use crate::document_tree::{
    HasChildren,
    elements as e,
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
    let structural_elems = pairs.map(block::convert_ssubel)
        .filter_map(|elem| match elem { Ok(Some(e)) => Some(Ok(e)), Err(e) => Some(Err(e)), Ok(None) => None })
        .collect::<Result<_,_>>()?;
    Ok(e::Document::with_children(structural_elems))
}
