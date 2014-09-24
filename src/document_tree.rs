///http://docutils.sourceforge.net/docs/ref/doctree.html
///serves as AST

#![feature(struct_inherit)]

//----------\\
//Categories\\
//----------\\

trait IElement; //TODO: needed?

///A document’s direct children
trait SubDocument:   IElement;
///body elements, topics, sidebars, transitions
trait SubStructure:  SubDocument;
trait SubSection:    IElement;
//title and body elements
trait SubTopic:      IElement;
trait SubSidebar:    IElement;
trait SubDLItem:     IElement;
trait SubField:      IElement;
trait SubOption:     IElement;
trait SubLineBlock:  IElement;
trait SubBlockQuote: IElement;
//label and body elements
trait SubFootnote:   IElement;

//-----------------\\
//Element hierarchy\\
//-----------------\\

virtual struct Element: IElement {
	ids: Vec<String>,
	names: Vec<String>,
	//left out dupnames
	source,
	classes,
};


//TODO: restructure so that elements with and without children derive from common structs

virtual struct BodyOrStructuralElement: Element;

virtual struct StructuralElement: BodyOrStructuralElement;
virtual struct StructuralSubElement: SubDocument;

virtual struct BodyElement: BodyOrStructuralElement, SubStructure, SubSection, SubTopic, SubSidebar, SubBlockQuote, SubFootnote;
virtual struct SimpleBodyElement: BodyElement;
virtual struct CompoundBodyElement: BodyElement;


virtual struct BodySubElement: Element;


virtual struct BibliographicElement: Element;


virtual struct DecorationElement: Element;


virtual struct TextOrInlineElement: Element;
virtual struct InlineElement: TextOrInlineElement;


//--------------\\
//Content Models\\
//--------------\\

trait HasChildren<C: Element> {
	fn add_child(C),
}

trait TextModel:        HasChildren<TextOrInlineElement>;
trait StructureModel:   HasChildren<SubStructure>;
trait DocumentModel:    HasChildren<SubDocument>;
trait InfoModel:        HasChildren<BibliographicElement>;
trait AuthorsModel:     HasChildren<AuthorInfo>;
trait DecorationModel:  HasChildren<DecorationElement>;
trait BodyModel:        HasChildren<BodyElement>;
trait SectionModel:     HasChildren<SubSection>;
trait TopicModel:       HasChildren<SubTopic>;
trait SidebarModel:     HasChildren<SubSidebar>;

trait ListModel:        HasChildren<ListItem>;

trait DLModel:          HasChildren<DefinitionListItem>;
trait DLItemModel:      HasChildren<SubDLItem>;

trait FieldListModel:   HasChildren<Field>;
trait FieldModel:       HasChildren<SubField>;

trait OptionListModel:  HasChildren<OptionListItem>;
trait OptionGroupModel: HasChildren<Option_>;
trait OptionModel:      HasChildren<SubOption>;

trait BlockQuoteModel:  HasChildren<SubBlockQuote>;
trait FootnoteModel:    HasChildren<SubFootnote>;
trait FigureModel:      HasChildren<SubFigure>;

//--------\\
//Elements\\
//--------\\

struct Document: StructuralElement, DocumentModel;
struct Section:  StructuralElement, SectionModel;
struct Topic:    StructuralElement, TopicModel,   SubStructure, SubSection, SubSidebar;
struct Sidebar:  StructuralElement, SidebarModel, SubStructure, SubSection;


struct Title:      StructuralSubElement, TextModel, SubSidebar, SubSection, SubTopic;
struct Subtitle:   StructuralSubElement, TextModel, SubSidebar;
struct Decoration: StructuralSubElement, DecorationModel;
struct Docinfo:    StructuralSubElement, InfoModel;
struct Transition: StructuralSubElement,            SubStructure, SubSection;


struct Author:       BibliographicElement, TextModel,    AuthorInfo;
struct Authors:      BibliographicElement, AuthorsModel;
struct Organization: BibliographicElement, TextModel,    AuthorInfo;
struct Address:      BibliographicElement, TextModel,    AuthorInfo { space: FixedSpace };
struct Contact:      BibliographicElement, TextModel,    AuthorInfo;
struct Version:      BibliographicElement, TextModel;
struct Revision:     BibliographicElement, TextModel;
struct Status:       BibliographicElement, TextModel;
struct Date:         BibliographicElement, TextModel;
struct Copyright:    BibliographicElement, TextModel;
struct Field:        BibliographicElement, FieldModel;


struct Header: DecorationElement, BodyModel;
struct Footer: DecorationElement, BodyModel;


struct Paragraph:              SimpleBodyElement, TextModel;
struct LiteralBlock:           SimpleBodyElement, TextModel { space: FixedSpace };
struct DoctestBlock:           SimpleBodyElement, TextModel { space: FixedSpace };
struct Image:                  SimpleBodyElement;
struct MathBlock:              SimpleBodyElement;
struct Raw:                    SimpleBodyElement { space: FixedSpace };
struct Rubric:                 SimpleBodyElement, TextModel;
struct Target:                 SimpleBodyElement, TextModel {} //TODO
struct SubstitutionDefinition: SimpleBodyElement, TextModel { ltrim: bool, rtrim: bool }
struct Comment:                SimpleBodyElement, TextModel { space: FixedSpace };
struct Pending:                SimpleBodyElement;


struct Compound:       CompoundBodyElement, BodyModel;
struct Container:      CompoundBodyElement, BodyModel;

struct BulletList:     CompoundBodyElement, ListModel { bullet: String };
struct EnumeratedList: CompoundBodyElement, ListModel { enumtype: EnumeratedListType, prefix: String, suffix: String }
struct DefinitionList: CompoundBodyElement, DLModel;
struct FieldList:      CompoundBodyElement, FieldListModel;
struct OptionList:     CompoundBodyElement, OptionListModel;

struct LineBlock:      CompoundBodyElement, LineBlockModel, SubLineBlock;
struct BlockQuote:     CompoundBodyElement, BlockQuoteModel;
struct Admonition:     CompoundBodyElement, TopicModel;
struct Attention:      CompoundBodyElement, BodyModel;
struct Hint:           CompoundBodyElement, BodyModel;
struct Note:           CompoundBodyElement, BodyModel;
struct Caution:        CompoundBodyElement, BodyModel;
struct Danger:         CompoundBodyElement, BodyModel;
struct Error:          CompoundBodyElement, BodyModel;
struct Important:      CompoundBodyElement, BodyModel;
struct Tip:            CompoundBodyElement, BodyModel;
struct Warning:        CompoundBodyElement, BodyModel;
struct Footnote:       CompoundBodyElement, FootnoteModel {} //TODO
struct Citation:       CompoundBodyElement, FootnoteModel {} //TODO
struct SystemMessage:  CompoundBodyElement, BodyModel { } //TODO
struct Figure:         CompoundBodyElement, FigureModel { width: uint } //TODO
struct Table:          CompoundBodyElement;


struct ListItem:           BodySubElement, BodyModel;

struct DefinitionListItem: BodySubElement, DLItemModel;
struct Term:               BodySubElement, TextModel, SubDLItem;
struct Classifier:         BodySubElement, TextModel, SubDLItem;
struct Definition:         BodySubElement, BodyModel, SubDLItem;

struct FieldName:          BodySubElement, TextModel, SubField;
struct FieldBody:          BodySubElement, BodyModel, SubField;

struct OptionList:         BodySubElement, OptionListModel;
struct OptionListItem:     BodySubElement, OptionListItemModel;
struct OptionGroup:        BodySubElement, OptionGroupModel;
struct Description:        BodySubElement, BodyModel;
struct Option_:            BodySubElement, OptionModel;
struct OptionString:       BodySubElement, TextModel, SubOption;
struct OptionArgument:     BodySubElement, TextModel, SubOption { delimiter: String };

struct Line:               BodySubElement, TextModel, SubLineBlock;
struct Attribution:        BodySubElement, TextModel, SubBlockQuote;
struct Label_:             BodySubElement, TextModel, SubFootnote;

struct Caption:            BodySubElement, TextModel, SubFigure;
struct Legend:             BodySubElement, BodyModel, SubFigure;

struct Abbreviation:          InlineElement;
struct Acronym:               InlineElement;
struct CitationReference:     InlineElement;
struct Emphasis:              InlineElement;
struct FootnoteReference:     InlineElement;
struct Generated:             InlineElement;
struct Image:                 InlineElement,          SubFigure;
struct Inline:                InlineElement;
struct Literal:               InlineElement;
struct Math:                  InlineElement;
struct Problematic:           InlineElement;
struct Reference:             InlineElement;
struct Strong:                InlineElement;
struct Subscript:             InlineElement;
struct SubstitutionReference: InlineElement;
struct Superscript:           InlineElement;
struct Target:                InlineElement;
struct TitleReference:        InlineElement;
struct Raw:                   InlineElement;

struct TextElement: TextOrInlineElement;


//------\\
//Others\\
//------\\


enum EnumeratedListType {
	Arabic,
	LowerAlpha,
	UpperAlpha,
	LowerRoman,
	UpperRoman,
}

enum FixedSpace { Default, Preserve }


//----\\
//impl\\
//----\\

impl Default for Element {
	fn default() -> Element
}

impl Field {
	get_name(&self) {
		let name_elem = self.children[0];
		assert!(name_elem.t)
	}
}
