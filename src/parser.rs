pub mod token;
pub mod conversion;
mod simplify;
mod pest_rst;
mod pair_ext_parse;
#[cfg(test)]
pub mod tests;

use failure::Error;
use pest::Parser;

use crate::document_tree::Document;

use self::pest_rst::{RstParser,Rule};
use self::conversion::convert_document;
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
