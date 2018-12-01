use url::Url;

use serde_derive::Serialize;

use super::serde_util::{serialize_url,serialize_opt_url};
use super::attribute_types::{FixedSpace,ID,NameToken,AlignHV,AlignH,Measure,EnumeratedListType};

pub trait ExtraAttributes<A> {
	fn with_extra(extra: A) -> Self;
	fn extra    (&    self) -> &    A;
	fn extra_mut(&mut self) -> &mut A;
}

macro_rules! impl_extra {
	( $name:ident { $( $(#[$pattr:meta])* $param:ident : $type:ty ),* $(,)* } ) => (
		impl_extra!(
			#[derive(Default,Debug,Serialize)]
			$name { $( $(#[$pattr])* $param : $type, )* }
		);
	);
	( $(#[$attr:meta])+ $name:ident { $( $(#[$pattr:meta])* $param:ident : $type:ty ),* $(,)* } ) => (
		$(#[$attr])+
		pub struct $name {
			$( $(#[$pattr])* pub $param : $type, )*
		}
	);
}

impl_extra!(Address { space: FixedSpace });
impl_extra!(LiteralBlock { space: FixedSpace });
impl_extra!(DoctestBlock { space: FixedSpace });
impl_extra!(SubstitutionDefinition { ltrim: Option<bool>, rtrim: Option<bool> });
impl_extra!(Comment { space: FixedSpace });
impl_extra!(Target {
	#[serde(serialize_with = "serialize_opt_url")]
	refuri: Option<Url>,
	refid: Option<ID>,
	refname: Vec<NameToken>,
	anonymous: bool,
});
impl_extra!(Raw { space: FixedSpace, format: Vec<NameToken> });
impl_extra!(#[derive(Debug,Serialize)] Image {
	#[serde(serialize_with = "serialize_url")]
	uri: Url,
	align: Option<AlignHV>,
	alt: Option<String>,
	height: Option<Measure>,
	width: Option<Measure>,
	scale: Option<u8>,
	#[serde(serialize_with = "serialize_opt_url")]
	target: Option<Url>,  // Not part of the DTD but a valid argument
});

//bools usually are XML yesorno. “auto” however either exists and is set to something random like “1” or doesn’t exist

impl_extra!(BulletList { bullet: Option<String> });
impl_extra!(EnumeratedList { enumtype: Option<EnumeratedListType>, prefix: Option<String>, suffix: Option<String> });

impl_extra!(Footnote { backrefs: Vec<ID>, auto: Option<bool> });
impl_extra!(Citation { backrefs: Vec<ID> });
impl_extra!(SystemMessage { backrefs: Vec<ID>, level: Option<usize>, line: Option<usize>, type_: Option<NameToken> });
impl_extra!(Figure { align: Option<AlignH>, width: Option<usize> });
impl_extra!(Table {}); //TODO

impl_extra!(OptionArgument { delimiter: Option<String> });

impl_extra!(Reference {
	name: Option<String>,
	#[serde(serialize_with = "serialize_opt_url")]
	refuri: Option<Url>,
	refid: Option<ID>,
	refname: Vec<NameToken>,
});
impl_extra!(FootnoteReference { refid: Option<ID>, refname: Vec<NameToken>, auto: Option<bool> });
impl_extra!(CitationReference { refid: Option<ID>, refname: Vec<NameToken> });
impl_extra!(SubstitutionReference { refname: Vec<NameToken> });
impl_extra!(Problematic { refid: Option<ID> });

//also have non-inline versions. Inline image is no figure child, inline target has content
impl_extra!(TargetInline {
	#[serde(serialize_with = "serialize_opt_url")]
	refuri: Option<Url>,
	refid: Option<ID>,
	refname: Vec<NameToken>,
	anonymous: bool,
});
impl_extra!(RawInline { space: FixedSpace, format: Vec<NameToken> });
pub type ImageInline = Image;

impl Image {
	pub fn new(uri: Url) -> Image { Image {
		uri: uri,
		align: None,
		alt: None,
		height: None,
		width: None,
		scale: None,
		target: None,
	} }
}
