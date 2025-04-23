#![warn(clippy::pedantic)]

mod html;

use std::io::Write;

use anyhow::{Error, anyhow};
use schemars::schema_for;

use document_tree::Document;

pub use html::render_html;

/// Render a document tree as JSON.
///
/// # Errors
/// Returns an error if serialization fails.
pub fn render_json<W>(document: &Document, stream: W) -> Result<(), Error>
where
    W: Write,
{
    serde_json::to_writer(stream, &document)?;
    Ok(())
}

#[expect(clippy::missing_panics_doc, reason = "infallible")]
/// Render the JSON schema for [`document_tree::Document`].
pub fn render_json_schema_document<W>(stream: W)
where
    W: Write,
{
    let schema = schema_for!(document_tree::Document);
    serde_json::to_writer(stream, &schema).unwrap();
}

/// Render a document tree as XML.
///
/// # Errors
/// Returns an error if serialization fails.
pub fn render_xml<W>(document: &Document, stream: W) -> Result<(), Error>
where
    W: Write,
{
    serde_xml_rs::to_writer(stream, &document)
        .map_err(|e| anyhow!("Failed to serialize XML: {}", e))?;
    Ok(())
}
