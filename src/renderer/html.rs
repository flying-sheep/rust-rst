use std::io::Write;

use failure::Error;

// use crate::url::Url;
use crate::document_tree::{
	Document,
	HasChildren,
	ExtraAttributes,
	elements as e,
	element_categories as c,
	// extra_attributes as a,
};


// static FOOTNOTE_SYMBOLS: [char; 10] = ['*', '†', '‡', '§', '¶', '#', '♠', '♥', '♦', '♣'];

pub fn render_html<W>(document: &Document, mut stream: W) -> Result<(), Error> where W: Write {
	document.render_html(stream.by_ref())
}

trait HTMLRender {
	fn render_html<W>(&self, stream: &mut W) -> Result<(), Error> where W: Write;
}

macro_rules! impl_html_render_cat {($cat:ident { $($member:ident),+ }) => {
	impl HTMLRender for c::$cat {
		fn render_html<W>(&self, stream: &mut W) -> Result<(), Error> where W: Write {
			match self {$(
				c::$cat::$member(elem) => (**elem).render_html(stream),
			)+}
		}
	}
}}

macro_rules! impl_html_render_simple {( $($type:ident => $tag:ident),+ ) => { $(
	impl HTMLRender for e::$type {
		fn render_html<W>(&self, stream: &mut W) -> Result<(), Error> where W: Write {
			write!(stream, "<{}>", stringify!($tag))?;
			for c in self.children() {
				(*c).render_html(stream)?;
			}
			write!(stream, "</{}>", stringify!($tag))?;
			Ok(())
		}
	}
)+ }}

macro_rules! impl_html_render_simple_nochildren {( $($type:ident => $tag:ident),+ ) => { $(
	impl HTMLRender for e::$type {
		fn render_html<W>(&self, stream: &mut W) -> Result<(), Error> where W: Write {
			write!(stream, "<{0}></{0}>", stringify!($tag))?;
			Ok(())
		}
	}
)+ }}

// Impl

impl HTMLRender for Document {
	fn render_html<W>(&self, stream: &mut W) -> Result<(), Error> where W: Write {
		write!(stream, "<!doctype html><html>")?;
		for c in self.children() {
			(*c).render_html(stream)?;
		}
		write!(stream, "</html>")?;
		Ok(())
	}
}

impl_html_render_cat!(StructuralSubElement { Title, Subtitle, Decoration, Docinfo, SubStructure });
impl_html_render_simple!(Title => h1, Subtitle => h2);

impl HTMLRender for e::Docinfo {
	fn render_html<W>(&self, _stream: &mut W) -> Result<(), Error> where W: Write {
		// Like “YAML frontmatter” in Markdown
		unimplemented!();
	}
}

impl HTMLRender for e::Decoration {
	fn render_html<W>(&self, _stream: &mut W) -> Result<(), Error> where W: Write {
		// Header or footer
		unimplemented!();
	}
}

impl_html_render_cat!(SubStructure { Topic, Sidebar, Transition, Section, BodyElement });
impl_html_render_simple!(Sidebar => aside, Section => section);
impl_html_render_simple_nochildren!(Transition => hr);

impl HTMLRender for e::Topic {
	fn render_html<W>(&self, _stream: &mut W) -> Result<(), Error> where W: Write {
		// A mini section with title
		unimplemented!();
	}
}

impl_html_render_cat!(BodyElement { Paragraph, LiteralBlock, DoctestBlock, MathBlock, Rubric, SubstitutionDefinition, Comment, Pending, Target, Raw, Image, Compound, Container, BulletList, EnumeratedList, DefinitionList, FieldList, OptionList, LineBlock, BlockQuote, Admonition, Attention, Hint, Note, Caution, Danger, Error, Important, Tip, Warning, Footnote, Citation, SystemMessage, Figure, Table });
impl_html_render_simple!(Paragraph => p, LiteralBlock => pre, MathBlock => math, Rubric => a, Compound => p, Container => div, BulletList => ul, EnumeratedList => ol, DefinitionList => dl, FieldList => dl, OptionList => pre, LineBlock => div, BlockQuote => blockquote, Admonition => aside, Attention => aside, Hint => aside, Note => aside, Caution => aside, Danger => aside, Error => aside, Important => aside, Tip => aside, Warning => aside, Figure => figure);
impl_html_render_simple_nochildren!(Image => img, Table => table);  //TODO: after implementing the table, move it to elems with children

impl HTMLRender for e::DoctestBlock {
	fn render_html<W>(&self, _stream: &mut W) -> Result<(), Error> where W: Write {
		// 
		unimplemented!();
	}
}

impl HTMLRender for e::SubstitutionDefinition {
	fn render_html<W>(&self, _stream: &mut W) -> Result<(), Error> where W: Write {
		// TODO: Should those be removed after resolving them
		Ok(())
	}
}

impl HTMLRender for e::Comment {
	fn render_html<W>(&self, stream: &mut W) -> Result<(), Error> where W: Write {
		write!(stream, "<!--")?;
		for c in self.children() {
			(*c).render_html(stream)?;
		}
		write!(stream, "-->")?;
		Ok(())
	}
}

impl HTMLRender for e::Pending {
	fn render_html<W>(&self, _stream: &mut W) -> Result<(), Error> where W: Write {
		// Will those be resolved by the time we get here?
		unimplemented!();
	}
}

impl HTMLRender for e::Target {
	fn render_html<W>(&self, _stream: &mut W) -> Result<(), Error> where W: Write {
		// Should be resolved by now
		Ok(())
	}
}

impl HTMLRender for e::Raw {
	fn render_html<W>(&self, stream: &mut W) -> Result<(), Error> where W: Write {
		for c in self.children() {
			write!(stream, "{}", c)?;
		}
		Ok(())
	}
}

impl HTMLRender for e::Footnote {
	fn render_html<W>(&self, _stream: &mut W) -> Result<(), Error> where W: Write {
		unimplemented!();
	}
}

impl HTMLRender for e::Citation {
	fn render_html<W>(&self, _stream: &mut W) -> Result<(), Error> where W: Write {
		unimplemented!();
	}
}

impl HTMLRender for e::SystemMessage {
	fn render_html<W>(&self, stream: &mut W) -> Result<(), Error> where W: Write {
		write!(stream, "<figure><caption>System Message</caption>")?;
		for c in self.children() {
			(*c).render_html(stream)?;
		}
		write!(stream, "</figure>")?;
		Ok(())
	}
}

impl_html_render_cat!(TextOrInlineElement { String, Emphasis, Strong, Literal, Reference, FootnoteReference, CitationReference, SubstitutionReference, TitleReference, Abbreviation, Acronym, Superscript, Subscript, Inline, Problematic, Generated, Math, TargetInline, RawInline, ImageInline });
impl_html_render_simple!(Emphasis => em, Strong => strong, Literal => code, FootnoteReference => a, CitationReference => a, TitleReference => a, Abbreviation => abbr, Acronym => acronym, Superscript => sup, Subscript => sub, Inline => span, Math => math, TargetInline => a);
impl_html_render_simple_nochildren!(ImageInline => img);

impl HTMLRender for String {
	fn render_html<W>(&self, stream: &mut W) -> Result<(), Error> where W: Write {
		write!(stream, "{}", self)?;
		Ok(())
	}
}

impl HTMLRender for e::Reference {
	fn render_html<W>(&self, stream: &mut W) -> Result<(), Error> where W: Write {
		let extra = self.extra();
		write!(stream, "<a")?;
		if let Some(ref target) = extra.refuri {
			write!(stream, " href=\"{}\"", target)?;
		}
		if let Some(ref name) = extra.name {
			write!(stream, " title=\"{}\"", name.0)?;
		}
		write!(stream, ">")?;
		for c in self.children() {
			(*c).render_html(stream)?;
		}
		write!(stream, "</a>")?;
		Ok(())
	}
}

impl HTMLRender for e::SubstitutionReference {
	fn render_html<W>(&self, _stream: &mut W) -> Result<(), Error> where W: Write {
		// Will those be resolved by the time we get here?
		unimplemented!();
	}
}

impl HTMLRender for e::Problematic {
	fn render_html<W>(&self, _stream: &mut W) -> Result<(), Error> where W: Write {
		// Broken inline markup leads to insertion of this in docutils
		unimplemented!();
	}
}

impl HTMLRender for e::Generated {
	fn render_html<W>(&self, _stream: &mut W) -> Result<(), Error> where W: Write {
		// Section numbers and so on
		unimplemented!();
	}
}

impl HTMLRender for e::RawInline {
	fn render_html<W>(&self, stream: &mut W) -> Result<(), Error> where W: Write {
		for c in self.children() {
			write!(stream, "{}", c)?;
		}
		Ok(())
	}
}


//--------------\\
//Content Models\\
//--------------\\

impl_html_render_cat!(SubTopic { Title, BodyElement });
impl_html_render_cat!(SubSidebar { Topic, Title, Subtitle, BodyElement });
impl_html_render_simple!(ListItem => li);

impl HTMLRender for e::DefinitionListItem {
	fn render_html<W>(&self, _stream: &mut W) -> Result<(), Error> where W: Write {
		// Term→dt, Definition→dd, Classifier→???
		unimplemented!();
	}
}

impl HTMLRender for e::Field {
	fn render_html<W>(&self, _stream: &mut W) -> Result<(), Error> where W: Write {
		// FieldName→dt, FieldBody→dd
		unimplemented!();
	}
}

impl HTMLRender for e::OptionListItem {
	fn render_html<W>(&self, _stream: &mut W) -> Result<(), Error> where W: Write {
		// OptionGroup→dt(s), Description→dd
		unimplemented!();
	}
}

impl_html_render_cat!(SubLineBlock { LineBlock, Line });

impl HTMLRender for e::Line {
	fn render_html<W>(&self, stream: &mut W) -> Result<(), Error> where W: Write {
		for c in self.children() {
			(*c).render_html(stream)?;
		}
		write!(stream, "<br>")?;
		Ok(())
	}
}

impl_html_render_cat!(SubBlockQuote { Attribution, BodyElement });
impl_html_render_simple!(Attribution => cite); //TODO: correct?

impl_html_render_cat!(SubFigure { Caption, Legend, BodyElement });
impl_html_render_simple!(Caption => caption);

impl HTMLRender for e::Legend {
	fn render_html<W>(&self, _stream: &mut W) -> Result<(), Error> where W: Write {
		unimplemented!();
	}
}

//------------\\
//Things to do\\
//------------\\

//TODO: prettyprint option list
//TODO: render admonitions: Admonition, Attention, Hint, Note, Caution, Danger, Error, Important, Tip, Warning
//TODO: properly render tables

//TODO: add reference target: Reference, FootnoteReference, CitationReference, TitleReference
//TODO: add title: Abbr, Acronym
//TODO: convert math, set display attr
//TODO: add id: Rubric, Target, TargetInline
//TODO: add src: Image, ImageInline
