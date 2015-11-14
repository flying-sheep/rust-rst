use url::Url;

use super::attribute_types::{FixedSpace,ID,NameToken,AlignHV,AlignH,Measure,EnumeratedListType};

pub trait ExtraAttributes<A> {
//	fn with_extra(extra: A) -> Self;
	fn extra    (&    self) -> &    A;
	fn extra_mut(&mut self) -> &mut A;
}

#[derive(Default,Debug)] pub struct Address { pub space: FixedSpace }
#[derive(Default,Debug)] pub struct LiteralBlock { pub space: FixedSpace }
#[derive(Default,Debug)] pub struct DoctestBlock { pub space: FixedSpace }
#[derive(Default,Debug)] pub struct SubstitutionDefinition { pub ltrim: Option<bool>, pub rtrim: Option<bool> }
#[derive(Default,Debug)] pub struct Comment { pub space: FixedSpace }
#[derive(Default,Debug)] pub struct Target { pub refuri: Option<Url>, pub refid: Option<ID>, pub refname: Vec<NameToken>, pub anonymous: Option<bool> }
#[derive(Default,Debug)] pub struct Raw { pub space: FixedSpace, pub format: Vec<NameToken> }
#[derive(Debug)]
pub struct Image {
	pub align: Option<AlignHV>,
	pub uri: Url,
	pub alt: Option<String>,
	pub height: Option<Measure>,
	pub width: Option<Measure>,
	pub scale: Option<f64>,
}

//bools usually are XML yesorno. “auto” however either exists and is set to something random like “1” or doesn’t exist

#[derive(Default,Debug)] pub struct BulletList { pub bullet: Option<String> }
#[derive(Default,Debug)] pub struct EnumeratedList { pub enumtype: Option<EnumeratedListType>, pub prefix: Option<String>, pub suffix: Option<String> }

#[derive(Default,Debug)] pub struct Footnote { pub backrefs: Vec<ID>, pub auto: Option<bool> }
#[derive(Default,Debug)] pub struct Citation { pub backrefs: Vec<ID> }
#[derive(Default,Debug)] pub struct SystemMessage { pub backrefs: Vec<ID>, pub level: Option<usize>, pub line: Option<usize>, pub type_: Option<NameToken> }
#[derive(Default,Debug)] pub struct Figure { pub align: Option<AlignH>, pub width: Option<usize> }
#[derive(Default,Debug)] pub struct Table; //TODO

#[derive(Default,Debug)] pub struct OptionArgument { pub delimiter: Option<String> }

#[derive(Default,Debug)] pub struct Reference { pub name: Option<String>, pub refuri: Option<Url>, pub refid: Option<ID>, pub refname: Vec<NameToken> }
#[derive(Default,Debug)] pub struct FootnoteReference { pub refid: Option<ID>, pub refname: Vec<NameToken>, pub auto: Option<bool> }
#[derive(Default,Debug)] pub struct CitationReference { pub refid: Option<ID>, pub refname: Vec<NameToken> }
#[derive(Default,Debug)] pub struct SubstitutionReference { pub refname: Vec<NameToken> }
#[derive(Default,Debug)] pub struct Problematic { pub refid: Option<ID> }

//also have non-inline versions. Inline image is no figure child, inline target has content
#[derive(Default,Debug)] pub struct TargetInline { pub refuri: Option<Url>, pub refid: Option<ID>, pub refname: Vec<NameToken>, pub anonymous: Option<bool> }
#[derive(Default,Debug)] pub struct RawInline { pub space: FixedSpace, pub format: Vec<NameToken> }
#[derive(Debug)]
pub struct ImageInline {
	pub align: Option<AlignHV>,
	pub uri: Url,
	pub alt: Option<String>,
	pub height: Option<Measure>,
	pub width: Option<Measure>,
	pub scale: Option<f64>,
}
