mod html;
pub mod opt;

use std::io::Write;

use anyhow::{anyhow, Error};

use document_tree::Document;

use opt::RenderOptions;

pub fn render_json<W, O>(document: &Document, stream: W, opts: O) -> Result<(), Error>
where
    W: Write,
    O: Into<RenderOptions>,
{
    let _: RenderOptions = opts.into();
    serde_json::to_writer(stream, &document)?;
    Ok(())
}

pub fn render_xml<W, O>(document: &Document, stream: W, opts: O) -> Result<(), Error>
where
    W: Write,
    O: Into<RenderOptions>,
{
    let _: RenderOptions = opts.into();
    serde_xml_rs::to_writer(stream, &document)
        .map_err(|e| anyhow!("Failed to serialize XML: {}", e))?;
    Ok(())
}

pub use html::render_html;
