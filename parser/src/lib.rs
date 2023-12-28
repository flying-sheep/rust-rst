mod conversion;
mod pair_ext_parse;
mod pest_rst;
mod simplify;
#[cfg(test)]
pub mod tests;
pub mod token;

use failure::Error;
use pest::Parser;

use document_tree::Document;

use self::conversion::convert_document;
use self::pest_rst::{RstParser, Rule};
use self::simplify::resolve_references;

/// Parse into a document tree and resolve sections, but not references.
pub fn parse_only(source: &str) -> Result<Document, Error> {
    let pairs = RstParser::parse(Rule::document, source)?;
    convert_document(pairs)
}

/// Parse into a document tree and resolve sections and references.
pub fn parse(source: &str) -> Result<Document, Error> {
    parse_only(source).map(resolve_references)
}
