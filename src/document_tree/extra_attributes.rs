use serde_derive::Serialize;

use crate::target;
use super::attribute_types::{FixedSpace,ID,NameToken,AlignHV,AlignH,Measure,EnumeratedListType};

pub trait ExtraAttributes<A> {
	fn with_extra(extra: A) -> Self;
	fn extra    (&    self) -> &    A;
	fn extra_mut(&mut self) -> &mut A;
}

/*
macro_rules! skip {
	(Option<$type:ty>) => { #[serde(skip_serializing_if = "Option::is_none")] };
	(Vec   <$type:ty>) => { #[serde(skip_serializing_if = "Vec::is_empty"  )] };
	(bool            ) => { #[serde(skip_serializing_if = "Not::not"       )] };
}
*/

macro_rules! impl_extra {
	( $name:ident { $( $(#[$pattr:meta])* $param:ident : $type:ty ),* $(,)* } ) => (
		impl_extra!(
			#[derive(Default,Debug,Serialize)]
			$name { $( $(#[$pattr])* $param : $type, )* }
		);
	);
	( $(#[$attr:meta])+ $name:ident { $( $(#[$pattr:meta])* $param:ident : $type:ty ),* $(,)* } ) => (
		$(#[$attr])+
		pub struct $name { $(
			$(#[$pattr])*
			// skip!($type)
			pub $param : $type,
		)* }
	);
}

impl_extra!(Address { space: FixedSpace });
impl_extra!(LiteralBlock { space: FixedSpace });
impl_extra!(DoctestBlock { space: FixedSpace });
impl_extra!(SubstitutionDefinition { ltrim: bool, rtrim: bool });
impl_extra!(Comment { space: FixedSpace });
impl_extra!(Target {
	refuri: Option<target::Target>,
	refid: Option<ID>,
	refname: Vec<NameToken>,
	anonymous: bool,
});
impl_extra!(Raw { space: FixedSpace, format: Vec<NameToken> });
impl_extra!(#[derive(Debug,Serialize)] Image {
	uri: target::Target,
	align: Option<AlignHV>,
	alt: Option<String>,
	height: Option<Measure>,
	width: Option<Measure>,
	scale: Option<u8>,
	target: Option<target::Target>,  // Not part of the DTD but a valid argument
});

//bools usually are XML yesorno. “auto” however either exists and is set to something random like “1” or doesn’t exist
//does auto actually mean the numbering prefix?

impl_extra!(BulletList { bullet: Option<String> });
impl_extra!(EnumeratedList { enumtype: Option<EnumeratedListType>, prefix: Option<String>, suffix: Option<String> });

impl_extra!(Footnote { backrefs: Vec<ID>, auto: bool });
impl_extra!(Citation { backrefs: Vec<ID> });
impl_extra!(SystemMessage { backrefs: Vec<ID>, level: Option<usize>, line: Option<usize>, type_: Option<NameToken> });
impl_extra!(Figure { align: Option<AlignH>, width: Option<usize> });
impl_extra!(Table {}); //TODO

impl_extra!(OptionArgument { delimiter: Option<String> });

impl_extra!(Reference {
	name: Option<String>,
	refuri: Option<target::Target>,
	refid: Option<ID>,
	refname: Vec<NameToken>,
});
impl_extra!(FootnoteReference { refid: Option<ID>, refname: Vec<NameToken>, auto: bool });
impl_extra!(CitationReference { refid: Option<ID>, refname: Vec<NameToken> });
impl_extra!(SubstitutionReference { refname: Vec<NameToken> });
impl_extra!(Problematic { refid: Option<ID> });

//also have non-inline versions. Inline image is no figure child, inline target has content
impl_extra!(TargetInline {
	refuri: Option<target::Target>,
	refid: Option<ID>,
	refname: Vec<NameToken>,
	anonymous: bool,
});
impl_extra!(RawInline { space: FixedSpace, format: Vec<NameToken> });
pub type ImageInline = Image;

impl Image {
	pub fn new(uri: target::Target) -> Image { Image {
		uri: uri,
		align: None,
		alt: None,
		height: None,
		width: None,
		scale: None,
		target: None,
	} }
}
