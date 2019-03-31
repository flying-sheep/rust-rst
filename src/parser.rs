pub mod token;
pub mod conversion;
mod pest_rst;
mod pair_ext_parse;
#[cfg(test)]
pub mod tests;

use failure::Error;
use pest::Parser;

use crate::document_tree::elements::Document;

use self::pest_rst::{RstParser,Rule};
use self::conversion::convert_document;


/// tokens to Document tree. resolves sections, but not references
pub fn parse(source: &str) -> Result<Document, Error> {
    let pairs = RstParser::parse(Rule::document, source)?;
    convert_document(pairs)
}
