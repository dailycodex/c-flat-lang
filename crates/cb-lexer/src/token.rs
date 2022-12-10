use std::fmt;

macro_rules! is_token {
    ($i:ident, $t:ident) => {
        pub fn $i(&self) -> bool {
            match self {
                Self::$t(_) => true,
                _ => false,
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Id(String),
    Int(String),
    Float(String),
    String(String),
    Char(String),
    Op(String),
    KeyWord(String),
    Error(String),
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(i) => write!(f, "{}", i),
            Self::Int(i) => write!(f, "{}", i),
            Self::Float(i) => write!(f, "{}", i),
            Self::String(s) => write!(f, "{}", s),
            Self::Char(c) => write!(f, "{}", c),
            Self::Op(o) => write!(f, "{}", o),
            Self::KeyWord(kw) => write!(f, "{}", kw),
            Self::Error(e) => write!(f, "unknown token: '{e}'"),
            Self::Eof => write!(f, "EOF"),
        }
    }
}

impl Token {
    pub fn lookup(name: &str) -> Option<Self> {
        match name {
            "fn" | "true" | "false" | "return" | "let" | "and" | "or" | "not" | "if" | "else" => {
                Some(Token::KeyWord(name.into()))
            }
            _ => None,
        }
    }

    is_token!(is_id, Id);
    is_token!(is_int, Int);
    is_token!(is_float, Float);
    is_token!(is_string, String);
    is_token!(is_char, Char);
    is_token!(is_op, Op);
    is_token!(is_keyword, KeyWord);
    pub fn is_eof(&self) -> bool {
        match self {
            Self::Eof => true,
            _ => false,
        }
    }
}
