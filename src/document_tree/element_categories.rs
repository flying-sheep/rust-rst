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

macro_rules! impl_into {
	([ $( (($subcat:ident :: $entry:ident), $supcat:ident), )+ ]) => {
		$( impl_into!($subcat::$entry => $supcat); )+
	};
	($subcat:ident :: $entry:ident => $supcat:ident ) => {
		impl Into<$supcat> for $entry {
			fn into(self) -> $supcat {
				$supcat::$subcat(Box::new(self.into()))
			}
		}
	};
}

macro_rules! synonymous_enum {
	( $subcat:ident : $($supcat:ident),+ ; $midcat:ident : $supsupcat:ident { $($entry:ident),+ $(,)* } ) => {
		synonymous_enum!($subcat : $( $supcat ),+ , $midcat { $($entry,)* });
		$( impl_into!($midcat::$entry => $supsupcat); )+
	};
	( $subcat:ident : $($supcat:ident),+ { $($entry:ident),+ $(,)* } ) => {
		synonymous_enum!($subcat { $( $entry, )* });
		cartesian!(impl_into, [ $( ($subcat::$entry) ),+ ], [ $($supcat),+ ]);
	};
	( $name:ident { $( $entry:ident ),+ $(,)* } ) => {
		#[derive(Serialize)]
		pub enum $name { $(
			$entry(Box<$entry>),
		)* }
		
		impl Debug for $name {
			fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
				match *self {
					$( $name::$entry(ref inner) => inner.fmt(fmt), )*
				}
			}
		}
		
		$( impl Into<$name> for $entry {
			fn into(self) -> $name {
				$name::$entry(Box::new(self))
			}
		} )*
	};
}

synonymous_enum!(StructuralSubElement { Title, Subtitle, Decoration, Docinfo, SubStructure });
synonymous_enum!(SubStructure: StructuralSubElement { Topic, Sidebar, Transition, Section, BodyElement });
synonymous_enum!(BodyElement: SubTopic, SubSidebar, SubBlockQuote, SubFootnote, SubFigure; SubStructure: StructuralSubElement {
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

synonymous_enum!(SubSection { Title, Subtitle, Docinfo, Decoration, SubStructure });
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
synonymous_enum!(SubFigure { Caption, Legend, BodyElement });

#[cfg(test)]
mod test {
	use std::default::Default;
	use super::*;
	
	#[test]
	fn test_convert_basic() {
		let _: BodyElement = Paragraph::default().into();
	}
	
	#[test]
	fn test_convert_more() {
		let _: SubStructure = Paragraph::default().into();
	}
	
	#[test]
	fn test_convert_even_more() {
		let _: StructuralSubElement = Paragraph::default().into();
	}
	
	#[test]
	fn test_convert_super() {
		let be: BodyElement = Paragraph::default().into();
		let _: StructuralSubElement = be.into();
	}
}
