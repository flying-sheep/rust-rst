use std::fmt::{self,Debug,Formatter};

use serde_derive::Serialize;

use super::elements::*;

pub trait HasChildren<C> {
	fn with_children(children: Vec<C>) -> Self;
	fn children(&self) -> &Vec<C>;
	fn children_mut(&mut self) -> &mut Vec<C>;
	fn append_child<R: Into<C>>(&mut self, child: R) {
		self.children_mut().push(child.into());
	}
	fn append_children<R: Into<C> + Clone>(&mut self, more: &[R]) {
		let children = self.children_mut();
		children.reserve(more.len());
		for child in more {
			children.push(child.clone().into());
		}
	}
}

macro_rules! synonymous_enum {( $name:ident { $( $entry:ident ),* } ) => (
	#[derive(Serialize)]
	pub enum $name {
		$( $entry($entry), )*
	}
	
	impl Debug for $name {
		fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
			match *self {
				$( $name::$entry(ref inner) => inner.fmt(fmt), )*
			}
		}
	}
	
	$(
		impl Into<$name> for $entry {
			fn into(self) -> $name {
				$name::$entry(self)
			}
		}
	)*
)}

synonymous_enum!(SubStructure { Topic, Sidebar, Transition, Section, BodyElement });
synonymous_enum!(StructuralSubElement { Title, Subtitle, Decoration, Docinfo, Transition, SubStructure });
synonymous_enum!(BodyElement {
	//Simple
	Paragraph, LiteralBlock, DoctestBlock, MathBlock, Rubric, SubstitutionDefinition, Comment, Pending, Target, Raw, Image,
	//Compound
	Compound, Container,
	BulletList, EnumeratedList, DefinitionList, FieldList, OptionList,
	LineBlock, BlockQuote, Admonition, Attention, Hint, Note, Caution, Danger, Error, Important, Tip, Warning, Footnote, Citation, SystemMessage, Figure, Table
});

synonymous_enum!(BibliographicElement { Author, Authors, Organization, Address, Contact, Version, Revision, Status, Date, Copyright, Field });

synonymous_enum!(TextOrInlineElement {
	String, Emphasis, Strong, Literal, Reference, FootnoteReference, CitationReference, SubstitutionReference, TitleReference, Abbreviation, Acronym, Superscript, Subscript, Inline, Problematic, Generated, Math,
	//also have non-inline versions. Inline image is no figure child, inline target has content
	TargetInline, RawInline, ImageInline
});

//--------------\\
//Content Models\\
//--------------\\

synonymous_enum!(SubSection { Title, Subtitle, Docinfo, Decoration, SubStructure, BodyElement });
synonymous_enum!(AuthorInfo { Author, Organization, Address, Contact });
synonymous_enum!(DecorationElement { Header, Footer });
synonymous_enum!(SubTopic { Title, BodyElement });
synonymous_enum!(SubSidebar { Topic, Title, Subtitle, BodyElement });
synonymous_enum!(SubDLItem { Term, Classifier, Definition });
synonymous_enum!(SubField { FieldName, FieldBody });
synonymous_enum!(SubOptionListItem { OptionGroup, Description });
synonymous_enum!(SubOption { OptionString, OptionArgument });
synonymous_enum!(SubLineBlock { LineBlock, Line });
synonymous_enum!(SubBlockQuote { Attribution, BodyElement });
synonymous_enum!(SubFootnote { Label, BodyElement });
synonymous_enum!(SubFigure { Image, Caption, Legend, BodyElement });
