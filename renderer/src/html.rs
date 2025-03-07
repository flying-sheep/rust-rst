mod elems_cats;
mod multi;
#[cfg(test)]
pub mod tests;

use std::io::Write;

use anyhow::Error;

// use crate::url::Url;
use document_tree::{Document, HasChildren};

// static FOOTNOTE_SYMBOLS: [char; 10] = ['*', '†', '‡', '§', '¶', '#', '♠', '♥', '♦', '♣'];

/// Render document as HTML
///
/// # Errors
/// Returns error if serialization fails
pub fn render_html<W>(document: &Document, stream: W, standalone: bool) -> Result<(), Error>
where
    W: Write,
{
    let mut renderer = HTMLRenderer { stream, level: 0 };
    if standalone {
        document.render_html(&mut renderer)
    } else {
        document.children().render_html(&mut renderer)
    }
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

struct HTMLRenderer<W>
where
    W: Write,
{
    stream: W,
    level: u8,
}

trait HTMLRender {
    fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write;
}

static HEAD: &str = r#"<head>
<meta charset="utf-8">
<meta name="color-scheme" content="dark light">
<meta name="viewport" content="width=device-width, initial-scale=1">
<style>
li.symbol {{ list-style-type: symbols(symbolic '*' '†' '‡' '§' '¶' '#' '♠' '♥' '♦' '♣'); }}
</style>
</head>"#;

impl HTMLRender for Document {
    fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        writeln!(renderer.stream, "<!doctype html>\n<html>\n{HEAD}\n<body>")?;
        self.children().render_html(renderer)?;
        writeln!(renderer.stream, "</body>\n</html>")?;
        Ok(())
    }
}

//------------\\
//Things to do\\
//------------\\

//TODO: prettyprint option list
//TODO: render admonitions: Admonition, Attention, Hint, Note, Caution, Danger, Error, Important, Tip, Warning
//TODO: properly render tables

//TODO: add reference target: FootnoteReference, CitationReference, TitleReference
//TODO: add title: Abbr, Acronym
//TODO: convert math, set display attr
//TODO: add id: Rubric, Target, TargetInline
