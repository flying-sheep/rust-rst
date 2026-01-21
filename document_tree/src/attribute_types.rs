use std::str::FromStr;

use anyhow::{Error, bail, format_err};
use linearize::Linearize;
use regex::Regex;
use schemars::JsonSchema;
use serde_derive::Serialize;

use crate::url::Url;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, JsonSchema)]
pub enum EnumeratedListType {
    Arabic,
    LowerAlpha,
    UpperAlpha,
    LowerRoman,
    UpperRoman,
}

#[derive(Clone, Copy, Linearize, Debug, PartialEq, Eq, Hash, Serialize, JsonSchema)]
pub enum FootnoteType {
    Number,
    Symbol,
}

impl TryFrom<char> for FootnoteType {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(FootnoteType::Number),
            '*' => Ok(FootnoteType::Symbol),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, Serialize, JsonSchema)]
pub enum FixedSpace {
    Default,
    // yes, default really is not “Default”
    #[default]
    Preserve,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, JsonSchema)]
pub enum AlignH {
    Left,
    Center,
    Right,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, JsonSchema)]
pub enum AlignHV {
    Top,
    Middle,
    Bottom,
    Left,
    Center,
    Right,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, JsonSchema)]
pub enum AlignV {
    Top,
    Middle,
    Bottom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, JsonSchema)]
pub enum TableAlignH {
    Left,
    Right,
    Center,
    Justify,
    Char,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, JsonSchema)]
pub enum TableBorder {
    Top,
    Bottom,
    TopBottom,
    All,
    Sides,
    None,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, JsonSchema)]
pub struct ID(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, JsonSchema)]
pub struct NameToken(pub String);

// The table DTD has the cols attribute of tgroup as required, but having
// TableGroupCols not implement Default would leave no possible implementation
// for TableGroup::with_children.
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, Serialize, JsonSchema)]
pub struct TableGroupCols(pub usize);

// no eq for f64
#[derive(Clone, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(tag = "unit", content = "value")]
#[schemars(_unstable_ref_variants)]
pub enum Measure {
    // https://docutils.sourceforge.net/docs/ref/rst/restructuredtext.html#length-units
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
        use self::AlignHV as A;
        Ok(match s {
            "top" => A::Top,
            "middle" => A::Middle,
            "bottom" => A::Bottom,
            "left" => A::Left,
            "center" => A::Center,
            "right" => A::Right,
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
        use self::Measure as M;
        let re =
            Regex::new(r"(?P<float>\d+\.\d*|\.?\d+)\s*(?P<unit>em|ex|mm|cm|in|px|pt|pc)").unwrap();
        let caps: regex::Captures = re
            .captures(s)
            .ok_or_else(|| format_err!("Invalid measure"))?;
        let value: f64 = caps["float"].parse()?;
        Ok(match &caps["unit"] {
            "em" => M::Em(value),
            "ex" => M::Ex(value),
            "mm" => M::Mm(value),
            "cm" => M::Cm(value),
            "in" => M::In(value),
            "px" => M::Px(value),
            "pt" => M::Pt(value),
            "pc" => M::Pc(value),
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn measure() {
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
impl_cannot_be_empty!(Url);
impl_cannot_be_empty!(TableGroupCols);

impl<T> CanBeEmpty for Option<T> {
    fn is_empty(&self) -> bool {
        self.is_none()
    }
}

impl<T> CanBeEmpty for Vec<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl CanBeEmpty for bool {
    fn is_empty(&self) -> bool {
        !self
    }
}

impl CanBeEmpty for FixedSpace {
    fn is_empty(&self) -> bool {
        self == &FixedSpace::default()
    }
}
