//https://docutils.sourceforge.net/docs/ref/rst/restructuredtext.html#bullet-lists

// *, +, -, •, ‣, ⁃
pub enum BulletListType {
    Ast,
    Plus,
    Minus,
    Bullet,
    TriBullet,
    HyphenBullet,
}
// 1, A, a, I, i
pub enum EnumListChar {
    Arabic,
    AlphaUpper,
    AlphaLower,
    RomanUpper,
    RomanLower,
    Auto,
}
// 1., (1), 1)
pub enum EnumListType {
    Period,
    ParenEnclosed,
    Paren,
}
// ! " # $ % & ' ( ) * + , - . / : ; < = > ? @ [ \ ] ^ _ ` { | } ~
pub enum AdornmentChar {
    Bang,
    DQuote,
    Hash,
    Dollar,
    Percent,
    Amp,
    SQuote,
    LParen,
    RParen,
    Ast,
    Plus,
    Comma,
    Minus,
    Period,
    Slash,
    Colon,
    Semicolon,
    Less,
    Eq,
    More,
    Question,
    At,
    LBrack,
    Backslash,
    RBrack,
    Caret,
    Underscore,
    Backtick,
    LBrace,
    Pipe,
    RBrace,
    Tilde,
}
// [1], [#], [*], [#foo]
pub enum FootnoteType {
    Numbered(usize),
    AutoNumber,
    AutoSymbol,
    AutoNamed(String),
}
