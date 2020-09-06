use std::path::PathBuf;
use serde_derive::Serialize;

use crate::attribute_types::{CanBeEmpty,ID,NameToken};
use crate::extra_attributes::{self,ExtraAttributes};
use crate::element_categories::*;


//-----------------\\
//Element hierarchy\\
//-----------------\\

pub trait Element {
	/// A list containing one or more unique identifier keys
	fn     ids    (&    self) -> &    Vec<ID>;
	fn     ids_mut(&mut self) -> &mut Vec<ID>;
	/// a list containing the names of an element, typically originating from the element's title or content.
	/// Each name in names must be unique; if there are name conflicts (two or more elements want to the same name),
	/// the contents will be transferred to the dupnames attribute on the duplicate elements.
	/// An element may have at most one of the names or dupnames attributes, but not both.
	fn   names    (&    self) -> &    Vec<NameToken>;
	fn   names_mut(&mut self) -> &mut Vec<NameToken>;
	fn  source    (&    self) -> &    Option<PathBuf>;
	fn  source_mut(&mut self) -> &mut Option<PathBuf>;
	fn classes    (&    self) -> &    Vec<String>;
	fn classes_mut(&mut self) -> &mut Vec<String>;
}

#[derive(Debug,Default,PartialEq,Serialize,Clone)]
pub struct CommonAttributes {
	#[serde(skip_serializing_if = "CanBeEmpty::is_empty")]
	ids: Vec<ID>,
	#[serde(skip_serializing_if = "CanBeEmpty::is_empty")]
	names: Vec<NameToken>,
	#[serde(skip_serializing_if = "CanBeEmpty::is_empty")]
	source: Option<PathBuf>,
	#[serde(skip_serializing_if = "CanBeEmpty::is_empty")]
	classes: Vec<String>,
	//TODO: dupnames
}

//----\\
//impl\\
//----\\

macro_rules! impl_element { ($name:ident) => (
	impl Element for $name {
		fn     ids    (&    self) -> &    Vec<ID>         { &    self.common.ids     }
		fn     ids_mut(&mut self) -> &mut Vec<ID>         { &mut self.common.ids     }
		fn   names    (&    self) -> &    Vec<NameToken>  { &    self.common.names   }
		fn   names_mut(&mut self) -> &mut Vec<NameToken>  { &mut self.common.names   }
		fn  source    (&    self) -> &    Option<PathBuf> { &    self.common.source  }
		fn  source_mut(&mut self) -> &mut Option<PathBuf> { &mut self.common.source  }
		fn classes    (&    self) -> &    Vec<String> { &    self.common.classes }
		fn classes_mut(&mut self) -> &mut Vec<String> { &mut self.common.classes }
	}
)}

macro_rules! impl_children { ($name:ident, $childtype:ident) => (
	impl HasChildren<$childtype> for $name {
		#[allow(clippy::needless_update)]
		fn with_children(children: Vec<$childtype>) -> $name { $name { children: children, ..Default::default() } }
		fn children    (&    self) -> &    Vec<$childtype> { &    self.children }
		fn children_mut(&mut self) -> &mut Vec<$childtype> { &mut self.children }
	}
)}

macro_rules! impl_extra { ($name:ident $($more:tt)*) => (
	impl ExtraAttributes<extra_attributes::$name> for $name {
		#[allow(clippy::needless_update)]
		fn with_extra(extra: extra_attributes::$name) -> $name { $name { common: Default::default(), extra: extra $($more)* } }
		fn extra    (&    self) -> &    extra_attributes::$name { &    self.extra }
		fn extra_mut(&mut self) -> &mut extra_attributes::$name { &mut self.extra }
	}
)}

trait HasExtraAndChildren<C, A> {
	fn with_extra_and_children(extra: A, children: Vec<C>) -> Self;
}

impl<T, C, A> HasExtraAndChildren<C, A> for T where T: HasChildren<C> + ExtraAttributes<A> {
	#[allow(clippy::needless_update)]
	fn with_extra_and_children(extra: A, mut children: Vec<C>) -> Self {
		let mut r = Self::with_extra(extra);
		r.children_mut().extend(children.drain(..));
		r
	}
}

macro_rules! impl_new {(
	$(#[$attr:meta])*
	pub struct $name:ident { $(
		$(#[$fattr:meta])*
		$field:ident : $typ:path
	),* $(,)* }
) => (
	$(#[$attr])*
	#[derive(Debug,PartialEq,Serialize,Clone)]
	pub struct $name { $( 
		$(#[$fattr])* $field: $typ,
	)* }
	impl $name {
		pub fn new( $( $field: $typ, )* ) -> $name { $name { $( $field: $field, )* } }
	}
)}

macro_rules! impl_elem {
	($name:ident) => {
		impl_new!(#[derive(Default)] pub struct $name {
			#[serde(flatten)] common: CommonAttributes,
		});
		impl_element!($name);
	};
	($name:ident; +) => {
		impl_new!(#[derive(Default)] pub struct $name {
			#[serde(flatten)] common: CommonAttributes,
			#[serde(flatten)] extra: extra_attributes::$name,
		});
		impl_element!($name); impl_extra!($name, ..Default::default());
	};
	($name:ident; *) => { //same as above with no default
		impl_new!(pub struct $name {
			#[serde(flatten)] common: CommonAttributes,
			#[serde(flatten)] extra: extra_attributes::$name
		});
		impl_element!($name); impl_extra!($name);
	};
	($name:ident, $childtype:ident) => {
		impl_new!(#[derive(Default)] pub struct $name {
			#[serde(flatten)] common: CommonAttributes,
			children: Vec<$childtype>,
		});
		impl_element!($name); impl_children!($name, $childtype);
	};
	($name:ident, $childtype:ident; +) => {
		impl_new!(#[derive(Default)] pub struct $name {
			#[serde(flatten)] common: CommonAttributes,
			#[serde(flatten)] extra: extra_attributes::$name,
			children: Vec<$childtype>,
		});
		impl_element!($name); impl_extra!($name, ..Default::default()); impl_children!($name, $childtype);
	};
}

macro_rules! impl_elems { ( $( ($($args:tt)*) )* ) => (
	$( impl_elem!($($args)*); )*
)}


#[derive(Default,Debug,Serialize)]
pub struct Document { children: Vec<StructuralSubElement> }
impl_children!(Document, StructuralSubElement);

impl_elems!(
	//structual elements
	(Section, StructuralSubElement)
	(Topic,   SubTopic)
	(Sidebar, SubSidebar)
	
	//structural subelements
	(Title,      TextOrInlineElement)
	(Subtitle,   TextOrInlineElement)
	(Decoration, DecorationElement)
	(Docinfo,    BibliographicElement)
	(Transition)
	
	//bibliographic elements
	(Author,       TextOrInlineElement)
	(Authors,      AuthorInfo)
	(Organization, TextOrInlineElement)
	(Address,      TextOrInlineElement; +)
	(Contact,      TextOrInlineElement)
	(Version,      TextOrInlineElement)
	(Revision,     TextOrInlineElement)
	(Status,       TextOrInlineElement)
	(Date,         TextOrInlineElement)
	(Copyright,    TextOrInlineElement)
	(Field,        SubField)
	
	//decoration elements
	(Header, BodyElement)
	(Footer, BodyElement)
	
	//simple body elements
	(Paragraph,              TextOrInlineElement)
	(LiteralBlock,           TextOrInlineElement; +)
	(DoctestBlock,           TextOrInlineElement; +)
	(MathBlock,              String)
	(Rubric,                 TextOrInlineElement)
	(SubstitutionDefinition, TextOrInlineElement; +)
	(Comment,                TextOrInlineElement; +)
	(Pending)
	(Target; +)
	(Raw, String; +)
	(Image; *)
	
	//compound body elements
	(Compound,  BodyElement)
	(Container, BodyElement)
	
	(BulletList,     ListItem; +)
	(EnumeratedList, ListItem; +)
	(DefinitionList, DefinitionListItem)
	(FieldList,      Field)
	(OptionList,     OptionListItem)
	
	(LineBlock,     SubLineBlock)
	(BlockQuote,    SubBlockQuote)
	(Admonition,    SubTopic)
	(Attention,     BodyElement)
	(Hint,          BodyElement)
	(Note,          BodyElement)
	(Caution,       BodyElement)
	(Danger,        BodyElement)
	(Error,         BodyElement)
	(Important,     BodyElement)
	(Tip,           BodyElement)
	(Warning,       BodyElement)
	(Footnote,      SubFootnote; +)
	(Citation,      SubFootnote; +)
	(SystemMessage, BodyElement; +)
	(Figure,        SubFigure;   +)
	(Table,         SubTable;    +)

	//table elements
	(TableGroup, SubTableGroup; +)
	(TableHead,  TableRow;      +)
	(TableBody,  TableRow;      +)
	(TableRow,   TableEntry;    +)
	(TableEntry, BodyElement;   +)
	(TableColspec; +)
	
	//body sub elements
	(ListItem, BodyElement)
	
	(DefinitionListItem, SubDLItem)
	(Term,               TextOrInlineElement)
	(Classifier,         TextOrInlineElement)
	(Definition,         BodyElement)
	
	(FieldName, TextOrInlineElement)
	(FieldBody, BodyElement)
	
	(OptionListItem, SubOptionListItem)
	(OptionGroup,    Option_)
	(Description,    BodyElement)
	(Option_,        SubOption)
	(OptionString,   String)
	(OptionArgument, String; +)
	
	(Line,        TextOrInlineElement)
	(Attribution, TextOrInlineElement)
	(Label,       TextOrInlineElement)
	
	(Caption, TextOrInlineElement)
	(Legend,  BodyElement)
	
	//inline elements
	(Emphasis,              TextOrInlineElement)
	(Literal,               String)
	(Reference,             TextOrInlineElement; +)
	(Strong,                TextOrInlineElement)
	(FootnoteReference,     TextOrInlineElement; +)
	(CitationReference,     TextOrInlineElement; +)
	(SubstitutionReference, TextOrInlineElement; +)
	(TitleReference,        TextOrInlineElement)
	(Abbreviation,          TextOrInlineElement)
	(Acronym,               TextOrInlineElement)
	(Superscript,           TextOrInlineElement)
	(Subscript,             TextOrInlineElement)
	(Inline,                TextOrInlineElement)
	(Problematic,           TextOrInlineElement; +)
	(Generated,             TextOrInlineElement)
	(Math,                  String)
	
	//also have non-inline versions. Inline image is no figure child, inline target has content
	(TargetInline, String; +)
	(RawInline,    String; +)
	(ImageInline; *)
	
	//text element = String
);

impl<'a> From<&'a str> for TextOrInlineElement {
	fn from(s: &'a str) -> Self {
		s.to_owned().into()
	}
}
