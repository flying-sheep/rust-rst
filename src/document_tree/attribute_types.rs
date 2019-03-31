use std::str::FromStr;

use failure::{Error,bail,format_err};
use serde_derive::Serialize;
use regex::Regex;

use crate::target;

#[derive(Debug,PartialEq,Eq,Hash,Serialize)]
pub enum EnumeratedListType {
	Arabic,
	LowerAlpha,
	UpperAlpha,
	LowerRoman,
	UpperRoman,
}

#[derive(Debug,PartialEq,Eq,Hash,Serialize)]
pub enum FixedSpace { Default, Preserve }  // yes, default really is not “Default”
impl Default for FixedSpace { fn default() -> FixedSpace { FixedSpace::Preserve } }

#[derive(Debug,PartialEq,Eq,Hash,Serialize)] pub enum AlignH { Left, Center, Right}
#[derive(Debug,PartialEq,Eq,Hash,Serialize)] pub enum AlignHV { Top, Middle, Bottom, Left, Center, Right }

#[derive(Debug,PartialEq,Eq,Hash,Serialize)] pub struct ID(pub String);
#[derive(Debug,PartialEq,Eq,Hash,Serialize)] pub struct NameToken(pub String);

// no eq for f64
#[derive(Debug,PartialEq,Serialize)]
pub enum Measure {  // http://docutils.sourceforge.net/docs/ref/rst/restructuredtext.html#length-units
	Em(f64),
	Ex(f64),
	Mm(f64),
	Cm(f64),
	In(f64),
	Px(f64),
	Pt(f64),
	Pc(f64),
}

impl FromStr for AlignHV {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use self::AlignHV::*;
		Ok(match s {
			"top"    => Top,
			"middle" => Middle,
			"bottom" => Bottom,
			"left"   => Left,
			"center" => Center,
			"right"  => Right,
			s => bail!("Invalid Alignment {}", s),
		})
	}
}

impl From<&str> for ID {
	fn from(s: &str) -> Self {
		ID(s.to_owned().replace(' ', "-"))
	}
}

impl From<&str> for NameToken {
	fn from(s: &str) -> Self {
		NameToken(s.to_owned())
	}
}

impl FromStr for Measure {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use self::Measure::*;
		let re = Regex::new(r"(?P<float>\d+\.\d*|\.?\d+)\s*(?P<unit>em|ex|mm|cm|in|px|pt|pc)").unwrap();
		let caps: regex::Captures = re.captures(s).ok_or_else(|| format_err!("Invalid measure"))?;
		let value: f64 = caps["float"].parse()?;
		Ok(match &caps["unit"] {
			"em" => Em(value),
			"ex" => Ex(value),
			"mm" => Mm(value),
			"cm" => Cm(value),
			"in" => In(value),
			"px" => Px(value),
			"pt" => Pt(value),
			"pc" => Pc(value),
			_ => unreachable!(),
		})
	}
}

#[cfg(test)]
mod test {
	use super::*;
	
	#[test]
	fn test_parse_measure() {
		let _a: Measure = "1.5em".parse().unwrap();
		let _b: Measure = "20 mm".parse().unwrap();
		let _c: Measure = ".5in".parse().unwrap();
		let _d: Measure = "1.pc".parse().unwrap();
	}
}

pub(crate) trait CanBeEmpty {
	fn is_empty(&self) -> bool;
}

/* Specialization necessary
impl<T> CanBeEmpty for T {
	fn is_empty(&self) -> bool { false }
}
*/
macro_rules! impl_cannot_be_empty {
	($t:ty) => {
		impl CanBeEmpty for $t {
			fn is_empty(&self) -> bool { false }
		}
	};
	($t:ty, $($ts:ty),*) => {
		impl_cannot_be_empty!($t);
		impl_cannot_be_empty!($($ts),*);
	};
}
impl_cannot_be_empty!(target::Target);

impl<T> CanBeEmpty for Option<T> {
	fn is_empty(&self) -> bool { self.is_none() }
}

impl<T> CanBeEmpty for Vec<T> {
	fn is_empty(&self) -> bool { self.is_empty() }
}

impl CanBeEmpty for bool {
	fn is_empty(&self) -> bool { !self }
}

impl CanBeEmpty for FixedSpace {
	fn is_empty(&self) -> bool { self == &FixedSpace::default() }
}

