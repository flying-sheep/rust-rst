#![warn(clippy::pedantic)]

mod conversion;
mod pair_ext_parse;
mod pest_rst;
#[cfg(test)]
pub mod tests;
pub mod token;
pub mod transforms;

use anyhow::Error;
use pest::Parser;

use document_tree::Document;

use self::conversion::convert_document;
use self::pest_rst::{RstParser, Rule};
use self::transforms::standard_transform;

/// Parse into a document tree and resolve sections, but not references.
///
/// # Errors
/// Returns an error if parsing fails.
pub fn parse_only(source: &str) -> Result<Document, Error> {
    let pairs = RstParser::parse(Rule::document, source)?;
    convert_document(pairs)
}

/// Parse into a document tree and resolve sections and references.
///
/// # Errors
/// Returns an error if parsing fails.
pub fn parse(source: &str) -> Result<Document, Error> {
    parse_only(source).map(standard_transform)
}
