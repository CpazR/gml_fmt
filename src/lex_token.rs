#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType<'a> {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Colon,
    Semicolon,
    Slash,
    Backslash,
    Star,
    Mod,
    Hashtag,
    Newline,

    ListIndexer,
    MapIndexer,
    GridIndexer,
    ArrayIndexer,

    Minus,
    Plus,
    Incrementer,
    Decrementer,
    Bang,
    Hook,

    LogicalAnd,
    LogicalOr,
    LogicalXor,
    BitAnd,
    BitOr,
    BitXor,
    BitLeft,
    BitRight,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Macro,
    RegionBegin,
    RegionEnd,
    Define,

    Var,
    If,
    Else,
    Return,
    For,
    Repeat,
    While,
    Do,
    Until,
    Switch,
    Case,
    DefaultCase,
    Break,
    Exit,
    True,
    False,
    Enum,

    AndAlias,
    OrAlias,
    XorAlias,
    NotAlias,
    ModAlias,
    Div,

    Identifier(&'a str),
    String(&'a str),
    Number(&'a str),

    Comment(&'a str),
    MultilineComment(&'a str),

    UnidentifiedInput(&'a str),
    EOF,
}

impl<'a> TokenType<'a> {
    pub fn is_ident(&self) -> bool {
        if let TokenType::Identifier(_) = self {
            return true;
        };
        false
    }
}

impl<'a> Display for TokenType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",
            TokenType::LeftBracket => "[",
            TokenType::RightBracket => "]",
            TokenType::Comma => ",",
            TokenType::Dot => ".",
            TokenType::Colon => ":",
            TokenType::Semicolon => ";",
            TokenType::Slash => "/",
            TokenType::Backslash => "\\",
            TokenType::Star => "*",
            TokenType::Mod => "%",
            TokenType::Hashtag => "#",
            TokenType::Newline => "\n",

            TokenType::ListIndexer => "[|",
            TokenType::MapIndexer => "[?",
            TokenType::GridIndexer => "[#",
            TokenType::ArrayIndexer => "[@",

            TokenType::Minus => "-",
            TokenType::Plus => "+",
            TokenType::Incrementer => "++",
            TokenType::Decrementer => "--",
            TokenType::Bang => "!",
            TokenType::Hook => "?",

            TokenType::LogicalAnd => "&&",
            TokenType::LogicalOr => "||",
            TokenType::LogicalXor => "^^",
            TokenType::BitAnd => "&",
            TokenType::BitOr => "|",
            TokenType::BitXor => "^",
            TokenType::BitLeft => "<<",
            TokenType::BitRight => ">>",
            TokenType::BangEqual => "!=",
            TokenType::Equal => "=",
            TokenType::EqualEqual => "==",
            TokenType::Greater => ">",
            TokenType::GreaterEqual => ">=",
            TokenType::Less => "<",
            TokenType::LessEqual => "<=",

            TokenType::Macro => "#macro",
            TokenType::RegionBegin => "#region",
            TokenType::RegionEnd => "#endregion",
            TokenType::Define => "#define",

            TokenType::Var => "var",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::Return => "return",
            TokenType::For => "for",
            TokenType::Repeat => "repeat",
            TokenType::While => "while",
            TokenType::Do => "do",
            TokenType::Until => "until",
            TokenType::Switch => "switch",
            TokenType::Case => "case",
            TokenType::DefaultCase => "default",
            TokenType::Break => "break",
            TokenType::Exit => "exit",
            TokenType::True => "true",
            TokenType::False => "false",
            TokenType::Enum => "enum",

            TokenType::AndAlias => "and",
            TokenType::OrAlias => "or",
            TokenType::XorAlias => "xor",
            TokenType::NotAlias => "not",
            TokenType::ModAlias => "mod",
            TokenType::Div => "div",

            TokenType::Identifier(literal) => literal,
            TokenType::String(literal) => literal,
            TokenType::Number(literal) => literal,

            TokenType::Comment(literal) => literal,
            TokenType::MultilineComment(literal) => literal,

            TokenType::UnidentifiedInput(literal) => literal,
            TokenType::EOF => "\n",
        })
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub line_number: u32,
    pub column_number: u32,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, line_number: u32, column_number: u32) -> Token {
        Token {
            token_type,
            line_number,
            column_number,
        }
    }

    pub fn print_name(&'a self) -> &str {
        match self.token_type {
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",
            TokenType::LeftBracket => "[",
            TokenType::RightBracket => "]",
            TokenType::Comma => ",",
            TokenType::Dot => ".",
            TokenType::Colon => ":",
            TokenType::Semicolon => ";",
            TokenType::Slash => "/",
            TokenType::Backslash => "\\",
            TokenType::Star => "*",
            TokenType::Mod => "%",
            TokenType::Hashtag => "#",
            TokenType::Newline => "\n",

            TokenType::ListIndexer => "[|",
            TokenType::MapIndexer => "[?",
            TokenType::GridIndexer => "[#",
            TokenType::ArrayIndexer => "[@",

            TokenType::Minus => "-",
            TokenType::Plus => "+",
            TokenType::Incrementer => "++",
            TokenType::Decrementer => "--",
            TokenType::Bang => "!",
            TokenType::Hook => "?",

            TokenType::LogicalAnd => "&&",
            TokenType::LogicalOr => "||",
            TokenType::LogicalXor => "^^",
            TokenType::BitAnd => "&",
            TokenType::BitOr => "|",
            TokenType::BitXor => "^",
            TokenType::BitLeft => "<<",
            TokenType::BitRight => ">>",
            TokenType::BangEqual => "!=",
            TokenType::Equal => "=",
            TokenType::EqualEqual => "==",
            TokenType::Greater => ">",
            TokenType::GreaterEqual => ">=",
            TokenType::Less => "<",
            TokenType::LessEqual => "<=",

            TokenType::Macro => "#macro",
            TokenType::RegionBegin => "#region",
            TokenType::RegionEnd => "#endregion",
            TokenType::Define => "#define",

            TokenType::Var => "var",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::Return => "return",
            TokenType::For => "for",
            TokenType::Repeat => "repeat",
            TokenType::While => "while",
            TokenType::Do => "do",
            TokenType::Until => "until",
            TokenType::Switch => "switch",
            TokenType::Case => "case",
            TokenType::DefaultCase => "default",
            TokenType::Break => "break",
            TokenType::Exit => "exit",
            TokenType::True => "true",
            TokenType::False => "false",
            TokenType::Enum => "enum",

            TokenType::AndAlias => "and",
            TokenType::OrAlias => "or",
            TokenType::XorAlias => "xor",
            TokenType::NotAlias => "not",
            TokenType::ModAlias => "mod",
            TokenType::Div => "div",

            TokenType::Identifier(literal) => literal,
            TokenType::String(literal) => literal,
            TokenType::Number(literal) => literal,

            TokenType::Comment(literal) => literal,
            TokenType::MultilineComment(literal) => literal,

            TokenType::UnidentifiedInput(literal) => literal,
            TokenType::EOF => "\n",
        }
    }
}

use std::fmt::{self, Display};
impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {:#?} on {}:{}.",
            self.token_type, self.line_number, self.column_number
        )
    }
}
