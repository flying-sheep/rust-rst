use url::Url;

use serde::{
	Serialize,
	Serializer,
	ser::SerializeStruct,
};

use super::attribute_types::{FixedSpace,ID,NameToken,AlignHV,AlignH,Measure,EnumeratedListType};

pub trait ExtraAttributes<A> {
//	fn with_extra(extra: A) -> Self;
	fn extra    (&    self) -> &    A;
	fn extra_mut(&mut self) -> &mut A;
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! ser_url {
	($self:ident, refuri      ) => { $self.refuri.as_ref().map(|uri| uri.to_string()) };
	($self:ident, uri         ) => { $self.uri.to_string() };
	($self:ident, $param:ident) => { $self.$param };
}

macro_rules! impl_extra {
	( $name:ident { $( $param:ident : $type:ty ),* $(,)* } ) => (
		impl_extra!(
			#[derive(Default,Debug)]
			$name { $( $param : $type, )* }
		);
	);
	( $(#[$attr:meta])+ $name:ident { $( $param:ident : $type:ty ),* $(,)* } ) => (
		$(#[$attr])+
		pub struct $name {
			$( pub $param : $type, )*
		}
		
		impl Serialize for $name {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
				#[allow(unused_mut)]
				let mut state = serializer.serialize_struct(stringify!($name), count!($($param)*))?;
				$( state.serialize_field(stringify!($param), &ser_url!(self, $param))?; )*
				state.end()
			}
		}
	);
}

impl_extra!(Address { space: FixedSpace });
impl_extra!(LiteralBlock { space: FixedSpace });
impl_extra!(DoctestBlock { space: FixedSpace });
impl_extra!(SubstitutionDefinition { ltrim: Option<bool>, rtrim: Option<bool> });
impl_extra!(Comment { space: FixedSpace });
impl_extra!(Target { refuri: Option<Url>, refid: Option<ID>, refname: Vec<NameToken>, anonymous: Option<bool> });
impl_extra!(Raw { space: FixedSpace, format: Vec<NameToken> });
impl_extra!(#[derive(Debug)] Image {
	align: Option<AlignHV>,
	uri: Url,
	alt: Option<String>,
	height: Option<Measure>,
	width: Option<Measure>,
	scale: Option<f64>,
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

impl_extra!(Reference { name: Option<String>, refuri: Option<Url>, refid: Option<ID>, refname: Vec<NameToken> });
impl_extra!(FootnoteReference { refid: Option<ID>, refname: Vec<NameToken>, auto: Option<bool> });
impl_extra!(CitationReference { refid: Option<ID>, refname: Vec<NameToken> });
impl_extra!(SubstitutionReference { refname: Vec<NameToken> });
impl_extra!(Problematic { refid: Option<ID> });

//also have non-inline versions. Inline image is no figure child, inline target has content
impl_extra!(TargetInline { refuri: Option<Url>, refid: Option<ID>, refname: Vec<NameToken>, anonymous: Option<bool> });
impl_extra!(RawInline { space: FixedSpace, format: Vec<NameToken> });
impl_extra!(#[derive(Debug)] ImageInline {
	align: Option<AlignHV>,
	uri: Url,
	alt: Option<String>,
	height: Option<Measure>,
	width: Option<Measure>,
	scale: Option<f64>,
});
