use std::io::Write;

use anyhow::{Error, bail};

// use crate::url::Url;
use super::{HTMLRender, HTMLRenderer, escape_html};
use document_tree::{
    Element, ExtraAttributes, HasChildren, attribute_types as at, element_categories as c,
    elements as e, extra_attributes as a,
};

// static FOOTNOTE_SYMBOLS: [char; 10] = ['*', '†', '‡', '§', '¶', '#', '♠', '♥', '♦', '♣'];

macro_rules! impl_html_render_cat {($cat:ident { $($member:ident),+ }) => {
    impl HTMLRender for c::$cat {
        fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
            match self {$(
                c::$cat::$member(elem) => elem.render_html(renderer),
            )+}
        }
    }
}}

macro_rules! impl_html_render_simple {
    (
        $type1:ident => $tag1:ident,
        $( $type:ident => $tag:ident ),+
    ) => {
        impl_html_render_simple!($type1 => $tag1);
        $( impl_html_render_simple!($type => $tag); )+
    };
    ( $type:ident => $tag:ident ) => {
        impl HTMLRender for e::$type {
            fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
                write!(renderer.stream, "<{}", stringify!($tag))?;
                if self.classes().len() > 0 {
                    write!(renderer.stream, " class=\"{}\"", self.classes().join(" "))?;
                }
                write!(renderer.stream, ">")?;
                self.children().render_html(renderer)?;
                write!(renderer.stream, "</{}>", stringify!($tag))?;
                Ok(())
            }
        }
    };
}

macro_rules! impl_html_render_simple_nochildren {( $($type:ident => $tag:ident),+ ) => { $(
    impl HTMLRender for e::$type {
        fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
            write!(renderer.stream, "<{0}></{0}>", stringify!($tag))?;
            Ok(())
        }
    }
)+ }}

// Impl

impl_html_render_cat!(StructuralSubElement {
    Title,
    Subtitle,
    Decoration,
    Docinfo,
    SubStructure
});
impl_html_render_simple!(Subtitle => h2);

impl HTMLRender for e::Title {
    fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        let level = if renderer.level > 6 {
            6
        } else {
            renderer.level
        };
        write!(renderer.stream, "<h{level}>")?;
        self.children().render_html(renderer)?;
        write!(renderer.stream, "</h{level}>")?;
        Ok(())
    }
}

impl HTMLRender for e::Docinfo {
    fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        // Like “YAML frontmatter” in Markdown
        unimplemented!();
    }
}

impl HTMLRender for e::Decoration {
    fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        // Header or footer
        unimplemented!();
    }
}

impl_html_render_cat!(SubStructure {
    Topic,
    Sidebar,
    Transition,
    Section,
    BodyElement
});
impl_html_render_simple!(Sidebar => aside);

impl HTMLRender for e::Section {
    fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        renderer.level += 1;
        write!(renderer.stream, "<section id=\"{0}\">", self.ids()[0].0)?;
        self.children().render_html(renderer)?;
        write!(renderer.stream, "</section>")?;
        renderer.level -= 1;
        Ok(())
    }
}

impl HTMLRender for e::Transition {
    fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        write!(renderer.stream, "<hr/>")?;
        Ok(())
    }
}

impl HTMLRender for e::Topic {
    fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        // A mini section with title
        unimplemented!();
    }
}

impl_html_render_cat!(BodyElement {
    Paragraph,
    LiteralBlock,
    DoctestBlock,
    MathBlock,
    Rubric,
    SubstitutionDefinition,
    Comment,
    Pending,
    Target,
    Raw,
    Image,
    Compound,
    Container,
    BulletList,
    EnumeratedList,
    DefinitionList,
    FieldList,
    OptionList,
    LineBlock,
    BlockQuote,
    Admonition,
    Attention,
    Hint,
    Note,
    Caution,
    Danger,
    Error,
    Important,
    Tip,
    Warning,
    Footnote,
    Citation,
    SystemMessage,
    Figure,
    Table
});
impl_html_render_simple!(Paragraph => p, MathBlock => math, Rubric => a, Compound => p, Container => div, BulletList => ul, EnumeratedList => ol, DefinitionList => dl, FieldList => dl, OptionList => pre, LineBlock => div, BlockQuote => blockquote, Admonition => aside, Attention => aside, Hint => aside, Note => aside, Caution => aside, Danger => aside, Error => aside, Important => aside, Tip => aside, Warning => aside, Figure => figure);
impl_html_render_simple_nochildren!(Table => table); //TODO: after implementing the table, move it to elems with children

// circumvent E0119
trait IMark {}
impl IMark for e::Image {}
impl IMark for e::ImageInline {}
impl<I> HTMLRender for I
where
    I: e::Element + a::ExtraAttributes<a::Image> + IMark,
{
    fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        let extra = self.extra();
        if let Some(target) = extra.target.as_ref() {
            write!(
                renderer.stream,
                "<a href=\"{}\">",
                escape_html(target.as_str())
            )?;
        }
        write!(renderer.stream, "<img")?;
        if let Some(alt) = extra.alt.as_ref() {
            write!(renderer.stream, " alt=\"{}\"", escape_html(alt))?;
        }
        // TODO: align: Option<AlignHV>
        // TODO: height: Option<Measure>
        // TODO: width: Option<Measure>
        // TODO: scale: Option<u8>
        write!(
            renderer.stream,
            " src=\"{}\" />",
            escape_html(extra.uri.as_str())
        )?;
        if extra.target.is_some() {
            write!(renderer.stream, "</a>")?;
        }
        Ok(())
    }
}

impl HTMLRender for e::LiteralBlock {
    fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        let mut cls_iter = self.classes().iter();
        let is_code = cls_iter.next() == Some(&"code".to_owned());
        write!(renderer.stream, "<pre>")?;
        if is_code {
            // TODO: support those classes not being at the start
            if let Some(lang) = cls_iter.next() {
                write!(renderer.stream, "<code class=\"language-{lang}\">")?;
            } else {
                write!(renderer.stream, "<code>")?;
            }
        }
        self.children().render_html(renderer)?;
        if is_code {
            write!(renderer.stream, "</code>")?;
        }
        write!(renderer.stream, "</pre>")?;
        Ok(())
    }
}

impl HTMLRender for e::DoctestBlock {
    fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        // TODO
        unimplemented!();
    }
}

impl HTMLRender for e::SubstitutionDefinition {
    fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        // TODO: Should those be removed after resolving them
        Ok(())
    }
}

impl HTMLRender for e::Comment {
    fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        write!(renderer.stream, "<!--")?;
        self.children().render_html(renderer)?;
        write!(renderer.stream, "-->")?;
        Ok(())
    }
}

impl HTMLRender for e::Pending {
    fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        // Will those be resolved by the time we get here?
        unimplemented!();
    }
}

impl HTMLRender for e::Target {
    fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        // Should be resolved by now
        Ok(())
    }
}

impl HTMLRender for e::Raw {
    fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        let extra = self.extra();
        if extra.format.contains(&at::NameToken("html".to_owned())) {
            for c in self.children() {
                write!(renderer.stream, "{c}")?;
            }
        }
        Ok(())
    }
}

impl HTMLRender for e::Footnote {
    fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        use c::SubFootnote::BodyElement;

        let mut children = self.children().iter();
        match (self.get_label(), self.extra().auto) {
            (Ok(label), Some(at::AutoFootnoteType::Symbol)) => {
                children.next(); // skip over the label
                write!(renderer.stream, "<li value=\"{label}\" class=\"symbol\">")?;
            }
            (Ok(label), _) => {
                children.next(); // skip over the label
                write!(renderer.stream, "<li value=\"{label}\">")?;
            }
            (Err(_), _) => {
                write!(renderer.stream, "<li>")?;
            }
        }
        for child in children {
            let BodyElement(child) = child else {
                bail!("Cannot have a footnote label anywhere but as first child node");
            };
            child.render_html(renderer)?;
        }
        write!(renderer.stream, "</li>")?;
        Ok(())
    }
}

impl HTMLRender for e::Citation {
    fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        unimplemented!();
    }
}

impl HTMLRender for e::SystemMessage {
    fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        write!(renderer.stream, "<figure><caption>System Message</caption>")?;
        self.children().render_html(renderer)?;
        write!(renderer.stream, "</figure>")?;
        Ok(())
    }
}

impl_html_render_cat!(TextOrInlineElement {
    String,
    Emphasis,
    Strong,
    Literal,
    Reference,
    FootnoteReference,
    CitationReference,
    SubstitutionReference,
    TitleReference,
    Abbreviation,
    Acronym,
    Superscript,
    Subscript,
    Inline,
    Problematic,
    Generated,
    Math,
    TargetInline,
    RawInline,
    ImageInline
});
impl_html_render_simple!(Emphasis => em, Strong => strong, Literal => code, FootnoteReference => a, CitationReference => a, TitleReference => a, Abbreviation => abbr, Acronym => acronym, Superscript => sup, Subscript => sub, Inline => span, Math => math, TargetInline => a);

impl HTMLRender for String {
    fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        write!(renderer.stream, "{}", escape_html(self))?;
        Ok(())
    }
}

impl HTMLRender for e::Reference {
    fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        let extra = self.extra();
        write!(renderer.stream, "<a")?;
        if let Some(target) = extra.refuri.as_ref() {
            write!(
                renderer.stream,
                " href=\"{}\"",
                escape_html(target.as_str())
            )?;
        }
        /*
        if let Some(name) = extra.name.as_ref() {
            write!(renderer.stream, " title=\"{}\"", escape_html(&name.0))?;
        }
        */
        write!(renderer.stream, ">")?;
        self.children().render_html(renderer)?;
        write!(renderer.stream, "</a>")?;
        Ok(())
    }
}

impl HTMLRender for e::SubstitutionReference {
    fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        // Will those be resolved by the time we get here?
        unimplemented!();
    }
}

impl HTMLRender for e::Problematic {
    fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        // Broken inline markup leads to insertion of this in docutils
        unimplemented!();
    }
}

impl HTMLRender for e::Generated {
    fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        // Section numbers and so on
        unimplemented!();
    }
}

impl HTMLRender for e::RawInline {
    fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        self.children().render_html(renderer)
    }
}

//--------------\\
//Content Models\\
//--------------\\

impl_html_render_cat!(SubTopic { Title, BodyElement });
impl_html_render_cat!(SubSidebar {
    Topic,
    Title,
    Subtitle,
    BodyElement
});
impl_html_render_simple!(ListItem => li);

impl HTMLRender for e::DefinitionListItem {
    fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        // Term→dt, Definition→dd, Classifier→???
        unimplemented!();
    }
}

impl HTMLRender for e::Field {
    fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        // FieldName→dt, FieldBody→dd
        unimplemented!();
    }
}

impl HTMLRender for e::OptionListItem {
    fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        // OptionGroup→dt(s), Description→dd
        unimplemented!();
    }
}

impl_html_render_cat!(SubLineBlock { LineBlock, Line });

impl HTMLRender for e::Line {
    fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        self.children().render_html(renderer)?;
        write!(renderer.stream, "<br>")?;
        Ok(())
    }
}

impl_html_render_cat!(SubBlockQuote {
    Attribution,
    BodyElement
});
impl_html_render_simple!(Attribution => cite); //TODO: correct?

impl_html_render_cat!(SubFigure {
    Caption,
    Legend,
    BodyElement
});
impl_html_render_simple!(Caption => caption);

impl HTMLRender for e::Legend {
    fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        unimplemented!();
    }
}
