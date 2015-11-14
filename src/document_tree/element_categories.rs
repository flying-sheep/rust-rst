use super::elements::*;

pub trait HasChildren<C> {
	fn add_child<R: Into<C>>(&mut self, R);
}

macro_rules! synonymous_enum {( $name:ident { $( $entry:ident ),* } ) => (
	#[derive(Debug)]
	pub enum $name {
		$(
			$entry($entry),
		)*
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
	TextElement, Emphasis, Strong, Literal, Reference, FootnoteReference, CitationReference, SubstitutionReference, TitleReference, Abbreviation, Acronym, Superscript, Subscript, Inline, Problematic, Generated, Math,
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
synonymous_enum!(SubFootnote { Label_, BodyElement });
synonymous_enum!(SubFigure { Image, Caption, Legend, BodyElement });
