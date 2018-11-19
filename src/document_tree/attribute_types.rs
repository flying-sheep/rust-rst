use serde_derive::Serialize;

#[derive(Debug,Serialize)]
pub enum EnumeratedListType {
	Arabic,
	LowerAlpha,
	UpperAlpha,
	LowerRoman,
	UpperRoman,
}

#[derive(Debug,Serialize)]
pub enum FixedSpace { Default, Preserve }  // yes, default really is not “Default”
impl Default for FixedSpace { fn default() -> FixedSpace { FixedSpace::Preserve } }

#[derive(Debug,Serialize)] pub enum AlignH { Left, Center, Right}
#[derive(Debug,Serialize)] pub enum AlignHV { Top, Middle, Bottom, Left, Center, Right }

#[derive(Debug,Serialize)] pub struct ID(String);
#[derive(Debug,Serialize)] pub struct NameToken(String);

#[derive(Debug,Serialize)]
pub enum Measure {
	Pixel(usize),
	Em(usize),
	//TODO
}
