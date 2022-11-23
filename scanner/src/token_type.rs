use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Id(String),
    Int(String),
    Ctrl(char),
    Op(String),
    KeyWord(KeyWord),
    RArrow,
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(i) => write!(f, "{}", i),
            Self::Int(i) => write!(f, "{}", i),
            Self::KeyWord(kw) => write!(f, "{}", kw),
            Self::Ctrl(c) => write!(f, "{}", c),
            Self::Op(o) => write!(f, "{}", o),
            Self::RArrow => write!(f, "->"),
            Self::Eof => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyWord {
    Fn,
    True,
    False,
    Return,
    Let,
    And,
    Or,
    Not,
    If,
    Else,
}

impl fmt::Display for KeyWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fn => write!(f, "fn"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Return => write!(f, "return"),
            Self::Let => write!(f, "let"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Not => write!(f, "not"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
        }
    }
}

impl KeyWord {
    pub fn lookup(name: &str) -> Option<Self> {
        use KeyWord::*;
        match name {
            "fn" => Some(Fn),
            "true" => Some(True),
            "false" => Some(False),
            "return" => Some(Return),
            "let" => Some(Let),
            "and" => Some(And),
            "or" => Some(Or),
            "not" => Some(Not),
            "if" => Some(If),
            "else" => Some(Else),
            _ => None,
        }
    }
}
