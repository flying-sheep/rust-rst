
// enum ElementType {
// 	//structual elements
// 	Section, Topic, Sidebar,
//
// 	//structural subelements
// 	Title, Subtitle, Decoration, Docinfo, Transition,
//
// 	//bibliographic elements
// 	Author, Authors, Organization,
// 	Address { space: FixedSpace },
// 	Contact, Version, Revision, Status,
// 	Date, Copyright, Field,
//
// 	//decoration elements
// 	Header, Footer,
//
// 	//simple body elements
// 	Paragraph,
// 	LiteralBlock { space: FixedSpace },
// 	DoctestBlock { space: FixedSpace },
// 	MathBlock, Rubric,
// 	SubstitutionDefinition { ltrim: bool, rtrim: bool },
// 	Comment { space: FixedSpace },
// 	Pending,
// 	Target { refuri: Url, refid: ID, refname: Vec<NameToken>, anonymous: bool },
// 	Raw { space: FixedSpace, format: Vec<NameToken> },
// 	Image {
// 		align: AlignHV,
// 		uri: Url,
// 		alt: String,
// 		height: Measure,
// 		width: Measure,
// 		scale: f64,
// 	},
//
// 	//compound body elements
// 	Compound, Container,
//
// 	BulletList { bullet: String },
// 	EnumeratedList { enumtype: EnumeratedListType, prefix: String, suffix: String },
// 	DefinitionList, FieldList, OptionList,
//
// 	LineBlock, BlockQuote,
// 	Admonition, Attention, Hint, Note,
// 	Caution, Danger, Error, Important,
// 	Tip, Warning,
// 	Footnote { backrefs: Vec<ID>, auto: bool },
// 	Citation { backrefs: Vec<ID> },
// 	SystemMessage { backrefs: Vec<ID>, level: usize, line: usize, type_: NameToken },
// 	Figure { align: AlignH, width: usize },
// 	Table, //TODO
//
// 	//body sub elements
// 	ListItem,
//
// 	DefinitionListItem, Term,
// 	Classifier, Definition,
//
// 	FieldName, FieldBody,
//
// 	OptionListItem, OptionGroup, Description, Option_, OptionString,
// 	OptionArgument { delimiter: String },
//
// 	Line, Attribution, Label,
//
// 	Caption, Legend,
//
// 	//inline elements
// 	Emphasis, Strong, Literal,
// 	Reference { name: String, refuri: Url, refid: ID, refname: Vec<NameToken> },
// 	FootnoteReference { refid: ID, refname: Vec<NameToken>, auto: bool },
// 	CitationReference { refid: ID, refname: Vec<NameToken> },
// 	SubstitutionReference { refname: Vec<NameToken> },
// 	TitleReference,
// 	Abbreviation, Acronym,
// 	Superscript, Subscript,
// 	Inline,
// 	Problematic { refid: ID },
// 	Generated, Math,
//
// 	//also have non-inline versions. Inline image is no figure child, inline target has content
// 	TargetInline { refuri: Url, refid: ID, refname: Vec<NameToken>, anonymous: bool },
// 	RawInline { space: FixedSpace, format: Vec<NameToken> },
// 	ImageInline {
// 		align: AlignHV,
// 		uri: Url,
// 		alt: String,
// 		height: Measure,
// 		width: Measure,
// 		scale: f64,
// 	},
//
// 	//text element
// 	TextElement,
// }
