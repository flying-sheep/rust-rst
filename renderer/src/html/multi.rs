use std::io::Write;

use anyhow::Error;

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
        impl HTMLRender for [$type] {
            fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
            where
                W: Write,
            {
                let many = self.len() > 1;
                if many {
                    write!(renderer.stream, $post)?;
                }
                for c in self {
                    c.render_html(renderer)?;
                    if many {
                        write!(renderer.stream, $post)?;
                    }
                }
                Ok(())
            }
        }
    };
}

// Impl

impl_html_render_multi!(
    c::StructuralSubElement["\n"],
    c::SubStructure["\n"],
    c::BodyElement["\n"],
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
    String
);
