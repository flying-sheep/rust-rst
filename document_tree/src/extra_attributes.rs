use serde_derive::Serialize;

use crate::attribute_types::{
    AlignH, AlignHV, AlignV, AutoFootnoteType, CanBeEmpty, EnumeratedListType, FixedSpace, ID,
    Measure, NameToken, TableAlignH, TableBorder, TableGroupCols,
};
use crate::url::Url;

pub trait ExtraAttributes<A> {
    fn with_extra(extra: A) -> Self;
    fn extra(&self) -> &A;
    fn extra_mut(&mut self) -> &mut A;
}

macro_rules! impl_extra {
    ( $name:ident { $( $(#[$pattr:meta])* $param:ident : $type:ty ),* $(,)* } ) => (
        impl_extra!(
            #[derive(Default,Debug,PartialEq,Serialize,Clone)]
            $name { $( $(#[$pattr])* $param : $type, )* }
        );
    );
    ( $(#[$attr:meta])+ $name:ident { $( $(#[$pattr:meta])* $param:ident : $type:ty ),* $(,)* } ) => (
        $(#[$attr])+
        pub struct $name { $(
            $(#[$pattr])*
            #[serde(skip_serializing_if = "CanBeEmpty::is_empty")]
            pub $param : $type,
        )* }
    );
}

impl_extra!(Address { space: FixedSpace });
impl_extra!(LiteralBlock { space: FixedSpace });
impl_extra!(DoctestBlock { space: FixedSpace });
impl_extra!(SubstitutionDefinition {
    ltrim: bool,
    rtrim: bool
});
impl_extra!(Comment { space: FixedSpace });
impl_extra!(Target {
    /// External reference to a URI/URL
    refuri: Option<Url>,
    /// References to ids attributes in other elements
    refid: Option<ID>,
    /// Internal reference to the names attribute of another element. May resolve to either an internal or external reference.
    refname: Vec<NameToken>,
    anonymous: bool,
});
impl_extra!(Raw { space: FixedSpace, format: Vec<NameToken> });
impl_extra!(#[derive(Debug,PartialEq,Serialize,Clone)] Image {
    uri: Url,
    align: Option<AlignHV>,
    alt: Option<String>,
    height: Option<Measure>,
    width: Option<Measure>,
    scale: Option<u8>,
    target: Option<Url>,  // Not part of the DTD but a valid argument
});

//bools usually are XML yesorno. “auto” however either exists and is set to something random like “1” or doesn’t exist
//does auto actually mean the numbering prefix?

impl_extra!(BulletList { bullet: Option<String> });
impl_extra!(EnumeratedList { enumtype: Option<EnumeratedListType>, prefix: Option<String>, suffix: Option<String> });

impl_extra!(Footnote { backrefs: Vec<ID>, auto: Option<AutoFootnoteType> });
impl_extra!(Citation { backrefs: Vec<ID> });
impl_extra!(SystemMessage { backrefs: Vec<ID>, level: Option<usize>, line: Option<usize>, type_: Option<NameToken> });
impl_extra!(Figure { align: Option<AlignH>, width: Option<usize> });
impl_extra!(Table { frame: Option<TableBorder>, colsep: Option<bool>, rowsep: Option<bool>, pgwide: Option<bool> });

impl_extra!(TableGroup { cols: TableGroupCols, colsep: Option<bool>, rowsep: Option<bool>, align: Option<TableAlignH> });
impl_extra!(TableHead { valign: Option<AlignV> });
impl_extra!(TableBody { valign: Option<AlignV> });
impl_extra!(TableRow { rowsep: Option<bool>, valign: Option<AlignV> });
impl_extra!(TableEntry { colname: Option<NameToken>, namest: Option<NameToken>, nameend: Option<NameToken>, morerows: Option<usize>, colsep: Option<bool>, rowsep: Option<bool>, align: Option<TableAlignH>, r#char: Option<char>, charoff: Option<usize>, valign: Option<AlignV>, morecols: Option<usize> });
impl_extra!(TableColspec { colnum: Option<usize>, colname: Option<NameToken>, colwidth: Option<String>, colsep: Option<bool>, rowsep: Option<bool>, align: Option<TableAlignH>, r#char: Option<char>, charoff: Option<usize>, stub: Option<bool> });

impl_extra!(OptionArgument { delimiter: Option<String> });

impl_extra!(Reference {
    name: Option<NameToken>,  //TODO: is CDATA in the DTD, so maybe no nametoken?
    /// External reference to a URI/URL
    refuri: Option<Url>,
    /// References to ids attributes in other elements
    refid: Option<ID>,
    /// Internal reference to the names attribute of another element
    refname: Vec<NameToken>,
});
impl_extra!(FootnoteReference { refid: Option<ID>, refname: Vec<NameToken>, auto: Option<AutoFootnoteType> });
impl_extra!(CitationReference { refid: Option<ID>, refname: Vec<NameToken> });
impl_extra!(SubstitutionReference { refname: Vec<NameToken> });
impl_extra!(Problematic { refid: Option<ID> });

//also have non-inline versions. Inline image is no figure child, inline target has content
impl_extra!(TargetInline {
    /// External reference to a URI/URL
    refuri: Option<Url>,
    /// References to ids attributes in other elements
    refid: Option<ID>,
    /// Internal reference to the names attribute of another element. May resolve to either an internal or external reference.
    refname: Vec<NameToken>,
    anonymous: bool,
});
impl_extra!(RawInline { space: FixedSpace, format: Vec<NameToken> });
pub type ImageInline = Image;

impl Image {
    #[must_use]
    pub fn new(uri: Url) -> Image {
        Image {
            uri,
            align: None,
            alt: None,
            height: None,
            width: None,
            scale: None,
            target: None,
        }
    }
}
