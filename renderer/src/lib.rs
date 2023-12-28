mod html;

use std::io::Write;

use failure::Error;

use document_tree::Document;

pub fn render_json<W>(document: &Document, stream: W) -> Result<(), Error>
where
    W: Write,
{
    serde_json::to_writer(stream, &document)?;
    Ok(())
}

pub fn render_xml<W>(document: &Document, stream: W) -> Result<(), Error>
where
    W: Write,
{
    serde_xml_rs::to_writer( stream, &document).map_err(failure::SyncFailure::new)?;
    Ok(())
}

pub use html::render_html;
