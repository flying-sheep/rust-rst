// rST does indentation for lists by using the column after the list item
// i’ll represent it as BulletList → Indent. e.g.:
//
//1. * foo
//   * bar
//
// becomes:
//
//EnumList(Arabic, Period) → Indent(3)
//   → BulletList(Asterisk) → Indent(2)
//     → Line("foo")
//   → Dedent(2)
//   → BulletList(Asterisk) → Indent(2)
//     → Line("bar")
//→ Dedent(5)

//http://docutils.sourceforge.net/docs/ref/rst/restructuredtext.html#bullet-lists

pub enum BulletListType { Ast, Plus, Minus, Bullet, TriBullet, HyphenBullet }
pub enum EnumListChar { Arabic, AlphaUpper, AlphaLower, RomanUpper, RomanLower, Auto }
pub enum EnumListType { Period, ParenEnclosed, Paren }
pub enum AdornmentChar {
	Bang, DQuote, Hash, Dollar, Percent, Amp, SQuote, LParen, RParen, Ast, Plus, Comma,
	Minus, Period, Slash, Colon, Semicolon, Less, Eq, More, Question, At, LBrack,
	Backslash, RBrack, Caret, Underscore, Backtick, LBrace, Pipe, RBrace, Tilde,
}
pub enum FootnoteType { Numbered(usize), AutoNumber, AutoSymbol, AutoNamed(String) }

pub enum TokenBlockLevel {
	EmptyLine,
	Indent(u8),  // plain indents mean blockquotes
	Dedent(u8),
	Line(String),
	
	Adornment(AdornmentChar, u32),  // ! " # $ % & ' ( ) * + , - . / : ; < = > ? @ [ \ ] ^ _ ` { | } ~
	// for a transition, this must be surrounded by blank lines, and be of length ≥ 4
	
	ListBulletItem(BulletListType),  // *, +, -, •, ‣, ⁃
	ListEnumItem(EnumListChar, EnumListType),  // 1, A, a, I, i; 1., (1), 1)
	ListDefinitionTerm(String, Option<String>),  //term and classifiers
	ListFieldName(String),
	ListOption(String),
	ListOptionArg(String),
	
	BlockLiteral,
	BlockQuotedLiteral(AdornmentChar),
	// line blocks use pipes (|)
	BlockDoctest(String),
	
	GridTableLine(String),
	GridTableRow(String),
	SimpleTableLine(String),
	
	Footnote(FootnoteType), // [1], [#], [*], [#foo]
	Citation(String),
	Directive(String, String),  // name and args
	SubstitutionDef(String, String),  // symbol and substitited line TODO: maybe only the former?
	Comment,
	CommentEmpty,  // if not followed by anything, “..” is special
	Target(String, String),
	TargetAnonymous(String),
}
