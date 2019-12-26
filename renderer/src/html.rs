#[cfg(test)]
pub mod tests;

use std::io::Write;

use failure::Error;

// use crate::url::Url;
use document_tree::{
	Document,Element,HasChildren,ExtraAttributes,
	elements as e,
	element_categories as c,
};


// static FOOTNOTE_SYMBOLS: [char; 10] = ['*', '†', '‡', '§', '¶', '#', '♠', '♥', '♦', '♣'];

pub fn render_html<W>(document: &Document, stream: W, standalone: bool) -> Result<(), Error> where W: Write {
	let mut renderer = HTMLRenderer { stream, level: 0 };
	if standalone {
		document.render_html(&mut renderer)
	} else {
		for c in document.children() {
			(*c).render_html(&mut renderer)?;
			writeln!(renderer.stream)?;
		}
		Ok(())
	}
}

fn escape_html(text: &str) -> String {
	text.replace('&', "&amp;")
		.replace('<', "&lt;")
		.replace('>', "&gt;")
		.replace('"', "&quot;")
}

struct HTMLRenderer<W> where W: Write {
	stream: W,
	level: u8,
}

trait HTMLRender {
	fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write;
}

macro_rules! impl_html_render_cat {($cat:ident { $($member:ident),+ }) => {
	impl HTMLRender for c::$cat {
		fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
			match self {$(
				c::$cat::$member(elem) => (**elem).render_html(renderer),
			)+}
		}
	}
}}

macro_rules! impl_html_render_simple {
	(
		$type1:ident => $tag1:ident $( [$($post1:tt)+] )?,
		$( $type:ident => $tag:ident $( [$($post:tt)+] )? ),+
	) => {
		impl_html_render_simple!($type1 => $tag1 $([$($post1)+])?);
		$( impl_html_render_simple!($type => $tag $([$($post)+])?); )+
	};
	( $type:ident => $tag:ident ) => {
		impl_html_render_simple!($type => $tag[""]);
	};
	( $type:ident => $tag:ident [ $post:expr ] ) => {
		impl HTMLRender for e::$type {
			fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
				let multiple_children = self.children().len() > 1;
				write!(renderer.stream, "<{}>", stringify!($tag))?;
				if multiple_children { write!(renderer.stream, $post)?; }
				for c in self.children() {
					(*c).render_html(renderer)?;
					if multiple_children { write!(renderer.stream, $post)?; }
				}
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

impl HTMLRender for Document {
	fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		writeln!(renderer.stream, "<!doctype html><html>")?;
		for c in self.children() {
			(*c).render_html(renderer)?;
			writeln!(renderer.stream)?;
		}
		writeln!(renderer.stream, "</html>")?;
		Ok(())
	}
}

impl_html_render_cat!(StructuralSubElement { Title, Subtitle, Decoration, Docinfo, SubStructure });
impl_html_render_simple!(Subtitle => h2);

impl HTMLRender for e::Title {
	fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		let level = if renderer.level > 6 { 6 } else { renderer.level };
		write!(renderer.stream, "<h{0}>", level)?;
		for c in self.children() {
			(*c).render_html(renderer)?;
		}
		write!(renderer.stream, "</h{0}>", level)?;
		Ok(())
	}
}

impl HTMLRender for e::Docinfo {
	fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		// Like “YAML frontmatter” in Markdown
		unimplemented!();
	}
}

impl HTMLRender for e::Decoration {
	fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		// Header or footer
		unimplemented!();
	}
}

impl_html_render_cat!(SubStructure { Topic, Sidebar, Transition, Section, BodyElement });
impl_html_render_simple!(Sidebar => aside);

impl HTMLRender for e::Section {
	fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		renderer.level += 1;
		writeln!(renderer.stream, "<section id=\"{0}\">", self.ids()[0].0)?;
		for c in self.children() {
			(*c).render_html(renderer)?;
			writeln!(renderer.stream)?;
		}
		write!(renderer.stream, "</section>")?;
		Ok(())
	}
}

impl HTMLRender for e::Transition {
	fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		write!(renderer.stream, "<hr/>")?;
		Ok(())
	}
}

impl HTMLRender for e::Topic {
	fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		// A mini section with title
		unimplemented!();
	}
}

impl_html_render_cat!(BodyElement { Paragraph, LiteralBlock, DoctestBlock, MathBlock, Rubric, SubstitutionDefinition, Comment, Pending, Target, Raw, Image, Compound, Container, BulletList, EnumeratedList, DefinitionList, FieldList, OptionList, LineBlock, BlockQuote, Admonition, Attention, Hint, Note, Caution, Danger, Error, Important, Tip, Warning, Footnote, Citation, SystemMessage, Figure, Table });
impl_html_render_simple!(Paragraph => p, LiteralBlock => pre, MathBlock => math, Rubric => a, Compound => p, Container => div, BulletList => ul["\n"], EnumeratedList => ol["\n"], DefinitionList => dl["\n"], FieldList => dl["\n"], OptionList => pre, LineBlock => div["\n"], BlockQuote => blockquote, Admonition => aside, Attention => aside, Hint => aside, Note => aside, Caution => aside, Danger => aside, Error => aside, Important => aside, Tip => aside, Warning => aside, Figure => figure);
impl_html_render_simple_nochildren!(Table => table);  //TODO: after implementing the table, move it to elems with children

//impl<I> HTMLRender for I where I: e::Element + a::ExtraAttributes<a::Image>
macro_rules! impl_render_html_image { ($t:ty) => { impl HTMLRender for $t {
	fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		let extra = self.extra();
		if let Some(ref target) = extra.target {
			write!(renderer.stream, "<a href=\"{}\">", escape_html(target.as_str()))?;
		}
		write!(renderer.stream, "<img")?;
		if let Some(ref alt) = extra.alt {
			write!(renderer.stream, " alt=\"{}\"", escape_html(alt))?;
		}
		// TODO: align: Option<AlignHV>
		// TODO: height: Option<Measure>
		// TODO: width: Option<Measure>
		// TODO: scale: Option<u8>
		write!(renderer.stream, " src=\"{}\" />", escape_html(extra.uri.as_str()))?;
		if extra.target.is_some() {
			write!(renderer.stream, "</a>")?;
		}
		Ok(())
	}
}}}
impl_render_html_image!(e::Image);
impl_render_html_image!(e::ImageInline);

impl HTMLRender for e::DoctestBlock {
	fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		// TODO
		unimplemented!();
	}
}

impl HTMLRender for e::SubstitutionDefinition {
	fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		// TODO: Should those be removed after resolving them
		Ok(())
	}
}

impl HTMLRender for e::Comment {
	fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		write!(renderer.stream, "<!--")?;
		for c in self.children() {
			(*c).render_html(renderer)?;
		}
		write!(renderer.stream, "-->")?;
		Ok(())
	}
}

impl HTMLRender for e::Pending {
	fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		// Will those be resolved by the time we get here?
		unimplemented!();
	}
}

impl HTMLRender for e::Target {
	fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		// Should be resolved by now
		Ok(())
	}
}

impl HTMLRender for e::Raw {
	fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		for c in self.children() {
			write!(renderer.stream, "{}", c)?;
		}
		Ok(())
	}
}

impl HTMLRender for e::Footnote {
	fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		unimplemented!();
	}
}

impl HTMLRender for e::Citation {
	fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		unimplemented!();
	}
}

impl HTMLRender for e::SystemMessage {
	fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		write!(renderer.stream, "<figure><caption>System Message</caption>")?;
		for c in self.children() {
			(*c).render_html(renderer)?;
		}
		write!(renderer.stream, "</figure>")?;
		Ok(())
	}
}

impl_html_render_cat!(TextOrInlineElement { String, Emphasis, Strong, Literal, Reference, FootnoteReference, CitationReference, SubstitutionReference, TitleReference, Abbreviation, Acronym, Superscript, Subscript, Inline, Problematic, Generated, Math, TargetInline, RawInline, ImageInline });
impl_html_render_simple!(Emphasis => em, Strong => strong, Literal => code, FootnoteReference => a, CitationReference => a, TitleReference => a, Abbreviation => abbr, Acronym => acronym, Superscript => sup, Subscript => sub, Inline => span, Math => math, TargetInline => a);

impl HTMLRender for String {
	fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		write!(renderer.stream, "{}", escape_html(self))?;
		Ok(())
	}
}

impl HTMLRender for e::Reference {
	fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		let extra = self.extra();
		write!(renderer.stream, "<a")?;
		if let Some(ref target) = extra.refuri {
			write!(renderer.stream, " href=\"{}\"", escape_html(target.as_str()))?;
		}
		/*
		if let Some(ref name) = extra.name {
			write!(renderer.stream, " title=\"{}\"", escape_html(&name.0))?;
		}
		*/
		write!(renderer.stream, ">")?;
		for c in self.children() {
			(*c).render_html(renderer)?;
		}
		write!(renderer.stream, "</a>")?;
		Ok(())
	}
}

impl HTMLRender for e::SubstitutionReference {
	fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		// Will those be resolved by the time we get here?
		unimplemented!();
	}
}

impl HTMLRender for e::Problematic {
	fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		// Broken inline markup leads to insertion of this in docutils
		unimplemented!();
	}
}

impl HTMLRender for e::Generated {
	fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		// Section numbers and so on
		unimplemented!();
	}
}

impl HTMLRender for e::RawInline {
	fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		for c in self.children() {
			write!(renderer.stream, "{}", c)?;
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
	fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		// Term→dt, Definition→dd, Classifier→???
		unimplemented!();
	}
}

impl HTMLRender for e::Field {
	fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		// FieldName→dt, FieldBody→dd
		unimplemented!();
	}
}

impl HTMLRender for e::OptionListItem {
	fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		// OptionGroup→dt(s), Description→dd
		unimplemented!();
	}
}

impl_html_render_cat!(SubLineBlock { LineBlock, Line });

impl HTMLRender for e::Line {
	fn render_html<W>(&self, renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		for c in self.children() {
			(*c).render_html(renderer)?;
		}
		write!(renderer.stream, "<br>")?;
		Ok(())
	}
}

impl_html_render_cat!(SubBlockQuote { Attribution, BodyElement });
impl_html_render_simple!(Attribution => cite); //TODO: correct?

impl_html_render_cat!(SubFigure { Caption, Legend, BodyElement });
impl_html_render_simple!(Caption => caption);

impl HTMLRender for e::Legend {
	fn render_html<W>(&self, _renderer: &mut HTMLRenderer<W>) -> Result<(), Error> where W: Write {
		unimplemented!();
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
