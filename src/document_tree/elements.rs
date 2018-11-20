use url::Url;
use serde::{
    Serialize,
    Serializer,
    ser::SerializeStruct,
};
use serde_derive::Serialize;

use super::extra_attributes::{self,ExtraAttributes};
use super::element_categories::*;


//-----------------\\
//Element hierarchy\\
//-----------------\\

pub trait Element {
	fn     ids    (&    self) -> &    Vec<String>;
	fn     ids_mut(&mut self) -> &mut Vec<String>;
	fn   names    (&    self) -> &    Vec<String>;
	fn   names_mut(&mut self) -> &mut Vec<String>;
	fn  source    (&    self) -> &    Option<Url>;
	fn  source_mut(&mut self) -> &mut Option<Url>;
	fn classes    (&    self) -> &    Vec<String>;
	fn classes_mut(&mut self) -> &mut Vec<String>;
}

#[derive(Default,Debug)]
pub struct CommonAttributes {
	ids:     Vec<String>,
	names:   Vec<String>,
	source:  Option<Url>,
	classes: Vec<String>,
	//left out dupnames
}

//----\\
//impl\\
//----\\

macro_rules! impl_element { ($name:ident) => (
	impl Element for $name {
		fn     ids    (&    self) -> &    Vec<String> { &    self.common.ids     }
		fn     ids_mut(&mut self) -> &mut Vec<String> { &mut self.common.ids     }
		fn   names    (&    self) -> &    Vec<String> { &    self.common.names   }
		fn   names_mut(&mut self) -> &mut Vec<String> { &mut self.common.names   }
		fn  source    (&    self) -> &    Option<Url> { &    self.common.source  }
		fn  source_mut(&mut self) -> &mut Option<Url> { &mut self.common.source  }
		fn classes    (&    self) -> &    Vec<String> { &    self.common.classes }
		fn classes_mut(&mut self) -> &mut Vec<String> { &mut self.common.classes }
	}
)}

macro_rules! impl_children { ($name:ident, $childtype:ident) => (
	impl HasChildren<$childtype> for $name {
		fn with_children(children: Vec<$childtype>) -> $name { $name { children: children, ..Default::default() } }
		fn children    (&    self) -> &    Vec<$childtype> { &    self.children }
		fn children_mut(&mut self) -> &mut Vec<$childtype> { &mut self.children }
	}
)}

macro_rules! impl_extra { ($name:ident) => (
	impl ExtraAttributes<extra_attributes::$name> for $name {
//		fn with_extra(extra: extra_attributes::$name) -> $name { $name { extra: extra, ..Default::default() } }
		fn extra    (&    self) -> &    extra_attributes::$name { &    self.extra }
		fn extra_mut(&mut self) -> &mut extra_attributes::$name { &mut self.extra }
	}
)}

macro_rules! impl_new {(
	$(#[$attr:meta])*
	pub struct $name:ident { $( $field:ident : $typ:path ),*
}) => (
	$(#[$attr])*
	pub struct $name { $( $field: $typ, )* }
	impl $name {
		pub fn new( $( $field: $typ, )* ) -> $name { $name { $( $field: $field, )* } }
	}
)}

macro_rules! impl_serialize {
	($name: ident, $extra: ident, $children: ident) => {
		impl Serialize for $name {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
				let mut state = serializer.serialize_struct(stringify!($name), 6)?;
				state.serialize_field("ids", self.ids())?;
				state.serialize_field("names", self.names())?;
				state.serialize_field("source", &self.source().as_ref().map(|uri| uri.to_string()))?;
				state.serialize_field("classes", self.classes())?;
				state.serialize_field("extra",    &impl_cond!($extra    ? self.extra()   ))?;
				state.serialize_field("children", &impl_cond!($children ? self.children()))?;
				state.end()
			}
		}
	};
}

macro_rules! impl_cond {
	(false ? $($b:tt)*) => { () };
	(true  ? $($b:tt)*) => { $($b)* };
}

macro_rules! impl_elem {
	($name:ident) => {
		impl_new!(#[derive(Default,Debug)] pub struct $name { common: CommonAttributes });
		impl_element!($name);
		impl_serialize!($name, false, false);
	};
	($name:ident; +) => {
		impl_new!(#[derive(Default,Debug)] pub struct $name { common: CommonAttributes, extra: extra_attributes::$name });
		impl_element!($name); impl_extra!($name);
		impl_serialize!($name, true, false);
	};
	($name:ident; *) => { //same as above with no default
		impl_new!(#[derive(Debug)] pub struct $name { common: CommonAttributes, extra: extra_attributes::$name });
		impl_element!($name); impl_extra!($name);
		impl_serialize!($name, true, false);
	};
	($name:ident, $childtype:ident) => {
		impl_new!(#[derive(Default,Debug)] pub struct $name { common: CommonAttributes, children: Vec<$childtype> });
		impl_element!($name); impl_children!($name, $childtype);
		impl_serialize!($name, false, true);
	};
	($name:ident, $childtype:ident; +) => {
		impl_new!(#[derive(Default,Debug)] pub struct $name { common: CommonAttributes, extra: extra_attributes::$name, children: Vec<$childtype> });
		impl_element!($name); impl_extra!($name); impl_children!($name, $childtype);
		impl_serialize!($name, true, true);
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
	(Section, SubSection)
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
	(Table; +) //TODO
	
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
	(Literal,               TextOrInlineElement)
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
	fn from(s: &'a str) -> TextOrInlineElement {
		s.to_owned().into()
	}
}
