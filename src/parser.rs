pub mod token;
pub mod conversion;
mod pest_rst;
mod pair_ext_parse;
#[cfg(test)]
pub mod tests;

use std::io::Write;

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


pub fn serialize_json<W>(source: &str, stream: W) -> Result<(), Error> where W: Write {
    let parsed = parse(source)?;
    serde_json::to_writer(stream, &parsed)?;
    Ok(())
}

pub fn serialize_xml<W>(source: &str, stream: W) -> Result<(), Error> where W: Write {
    let parsed = parse(source)?;
    serde_xml_rs::to_writer(stream, &parsed).map_err(failure::SyncFailure::new)?;
    Ok(())
}
