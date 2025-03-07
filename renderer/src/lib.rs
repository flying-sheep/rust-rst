#![warn(clippy::pedantic)]

mod html;

use std::io::Write;

use anyhow::{Error, anyhow};

use document_tree::Document;

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

pub use html::render_html;
