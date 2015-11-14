#[derive(Debug)]
pub enum EnumeratedListType {
	Arabic,
	LowerAlpha,
	UpperAlpha,
	LowerRoman,
	UpperRoman,
}

#[derive(Debug)]
pub enum FixedSpace { Default, Preserve }  // yes, default really is not “Default”
impl Default for FixedSpace { fn default() -> FixedSpace { FixedSpace::Preserve } }

#[derive(Debug)] pub enum AlignH { Left, Center, Right}
#[derive(Debug)] pub enum AlignHV { Top, Middle, Bottom, Left, Center, Right }

#[derive(Debug)] pub struct ID(String);
#[derive(Debug)] pub struct NameToken(String);

#[derive(Debug)]
pub enum Measure {
	Pixel(usize),
	Em(usize),
	//TODO
}
