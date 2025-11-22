use std::{borrow::Borrow, io::Write};

use anyhow::Result;

use super::{HTMLRender, HTMLRenderer};

use document_tree::{element_categories as c, elements as e};

macro_rules! impl_html_render_multi {
    (
        $type1:path $( [$($post1:tt)+] )?,
        $( $type:path $( [$($post:tt)+] )? ),+
    ) => {
        impl_html_render_multi!($type1 $([$($post1)+])?);
        $( impl_html_render_multi!($type $([$($post)+])?); )+
    };
    ( $type:path ) => {
        impl_html_render_multi!($type[""]);
    };
    ( $type:path [ $post:expr ] ) => {
        impl HTMLRender for [&$type] {
            fn render_html<W: Write>(&self, renderer: &mut HTMLRenderer<W>) -> Result<()> {
                write_optional_newlines::<$type, _, _>(renderer, self, $post)
            }
        }
        impl HTMLRender for [$type] {
            fn render_html<W: Write>(&self, renderer: &mut HTMLRenderer<W>) -> Result<()> {
                write_optional_newlines::<$type, _, _>(renderer, self, $post)
            }
        }
    };
}

fn write_optional_newlines<R, E, W>(
    renderer: &mut HTMLRenderer<W>,
    elems: &[E],
    post: &str,
) -> Result<()>
where
    R: HTMLRender,
    E: Borrow<R>,
    W: Write,
{
    let many = elems.len() > 1;
    if many {
        write!(renderer.stream, "{post}")?;
    }
    for c in elems {
        c.borrow().render_html(renderer)?;
        if many {
            write!(renderer.stream, "{post}")?;
        }
    }
    Ok(())
}

macro_rules! impl_html_render_multi_body {
    ( $type1:path, $( $type:path ),+ ) => {
        impl_html_render_multi_body!($type1);
        $( impl_html_render_multi_body!($type); )+
    };
    ( $type:path ) => {
        impl HTMLRender for [$type] {
            fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<()>
            where
                W: Write,
            {
                let many = self.len() > 1;
                if many {
                    writeln!(renderer.stream)?;
                }
                let mut footnotes: Vec<&e::Footnote> = vec![];
                for c in self {
                    if let Ok(&c::BodyElement::Footnote(ref f)) = c.try_into() {
                        footnotes.push(f.as_ref());
                        continue;
                    }
                    write_footnotes(renderer, &footnotes)?;
                    c.render_html(renderer)?;
                    if many {
                        writeln!(renderer.stream)?;
                    }
                }
                write_footnotes(renderer, &footnotes)?;
                Ok(())
            }
        }
    };
}

fn write_footnotes<W>(renderer: &mut HTMLRenderer<W>, footnotes: &[&e::Footnote]) -> Result<()>
where
    W: Write,
{
    if footnotes.is_empty() {
        return Ok(());
    }
    writeln!(renderer.stream, "<ol class=\"footnotes\">")?;
    for f in footnotes {
        f.render_html(renderer)?;
        writeln!(renderer.stream)?;
    }
    writeln!(renderer.stream, "</ol>")?;
    Ok(())
}

// Impl

impl_html_render_multi!(
    c::TextOrInlineElement,
    c::SubSidebar,
    c::SubLineBlock,
    c::SubBlockQuote,
    c::SubTopic,
    c::SubFigure,
    e::ListItem["\n"],
    e::DefinitionListItem,
    e::Field,
    e::OptionListItem,
    e::Footnote["\n"],
    String
);

impl_html_render_multi_body!(c::StructuralSubElement, c::SubStructure, c::BodyElement);
