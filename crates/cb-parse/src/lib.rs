use cb_lexer::{Scanner, Span, Token};
use std::fmt;
use std::iter::Peekable;

type CResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
enum ParserError {
    BadToken(Option<(Token, Span)>),
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadToken(Some((t, s))) => write!(f, "{}:{} {t:?}", s.start, s.end),
            Self::BadToken(None) => write!(f, "BadToken::None"),
        }
    }
}

impl std::error::Error for ParserError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    None,
    Primary,
    Term,   // + -
    Factor, // * /
    // Or,         // or
    // And,        // and
    // Equality,   // == !=
    Comparison, // < > <= >=
    Assignment, // =
    // Call,       // . ()
    // Func,
    Unary, // ! -
}

impl From<Token> for Precedence {
    fn from(token: Token) -> Self {
        match token {
            Token::Id(_) => Self::None,
            Token::Int(_) => Self::None,
            Token::Float(_) => Self::None,
            Token::String(_) => Self::None,
            Token::Char(_) => Self::None,
            Token::Op(ref op) => match op.as_str() {
                "+" | "-" => Self::Term,
                "*" | "/" => Self::Factor,
                ">" | "<" | ">=" | "<=" | "==" | "!=" => Self::Comparison,
                "=" => Self::Assignment,
                _ => Self::None,
            },
            Token::KeyWord(ref b) if b == "true" || b == "false" => Self::Primary,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Minus,
    Plus,
    Mult,
    Div,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minus => write!(f, "-"),
            Self::Plus => write!(f, "+"),
            Self::Mult => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
}

impl TryFrom<Token> for Op {
    type Error = &'static str;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Op(ref op) => Self::try_from(op),
            _ => Err("not an operator"),
        }
    }
}

impl TryFrom<&str> for Op {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "-" => Ok(Self::Minus),
            "+" => Ok(Self::Plus),
            "*" => Ok(Self::Mult),
            "/" => Ok(Self::Div),
            _ => Err("not an operator"),
        }
    }
}

impl TryFrom<&String> for Op {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Int(i32),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{i}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Atom(Atom),
    Unary(Op, Box<Self>),
    Binary(Op, Box<Self>, Box<Self>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(i) => write!(f, "{i}"),
            Self::Unary(op, expr) => write!(f, "({op} {expr})"),
            Self::Binary(op, lhs, rhs) => write!(f, "({op} {lhs} {rhs})"),
        }
    }
}

pub fn parse(src: &str) -> CResult<Vec<Expr>> {
    let lexer = Scanner::new(src);
    let mut parser = Parser::new(lexer.peekable());
    parser.parse()
}

struct Parser<'a> {
    lexer: Peekable<Scanner<'a>>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Peekable<Scanner<'a>>) -> Self {
        Self { lexer }
    }

    fn is_end(&mut self) -> bool {
        self.lexer.peek().map(|_| false).unwrap_or(true)
    }

    fn program(&mut self) -> CResult<Expr> {
        self.expression(Precedence::None)
    }

    fn expression(&mut self, min_bp: Precedence) -> CResult<Expr> {
        let mut lhs = match dbg!(self.lexer.next()) {
            Some((Token::Int(a), _)) => Expr::Atom(Atom::Int(a.parse().unwrap())),
            Some((Token::Op(ref op), _)) if op == "(" => {
                let lhs = self.expression(Precedence::None)?;
                self.lexer.next();
                lhs
            }
            Some((Token::Op(ref op), _)) if op == "-" => {
                let op = Op::try_from(op.as_str())?;
                let rhs = self.expression(Precedence::Unary)?;
                Expr::Unary(op, Box::new(rhs))
            }
            t => return Err(Box::new(ParserError::BadToken(t))),
        };
        loop {
            let Some((token, _)) = self.lexer.next() else {
                break;
            };
            let bp = Precedence::from(token.clone());
            let op = match Op::try_from(token.clone()) {
                Ok(o) => o,
                Err(_) => break,
            };
            if bp < min_bp {
                break;
            }
            match bp {
                Precedence::Term | Precedence::Factor => {
                    let rhs = self.expression(bp)?;
                    lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
                }
                _ => {}
            }
        }
        Ok(lhs)
    }

    fn parse(&mut self) -> CResult<Vec<Expr>> {
        let mut result = vec![];
        while !self.is_end() {
            let e = self.program()?;
            result.push(e);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn into_string<E: fmt::Display>(i: &mut impl Iterator<Item = E>) -> String {
        i.next().map(|t| t.to_string()).unwrap_or("".into())
    }

    #[test]
    fn expressions() {
        let exprs = parse("1").unwrap_or(vec![]);
        let mut exprs = exprs.iter();
        assert_eq!(into_string(&mut exprs), "1");
    }

    #[test]
    fn change_precedence() {
        let exprs = parse("(1)").unwrap_or(vec![]);
        let mut exprs = exprs.iter();
        assert_eq!(into_string(&mut exprs), "1");
    }

    #[test]
    fn unary() {
        let exprs = parse("-1").unwrap_or(vec![]);
        let mut exprs = exprs.iter();
        assert_eq!(into_string(&mut exprs), "(- 1)");
    }

    #[test]
    fn binary() {
        let exprs = parse("1 + 1").unwrap_or(vec![]);
        let mut exprs = exprs.iter();
        assert_eq!(into_string(&mut exprs), "(+ 1 1)");

        let exprs = parse("1 + 2 * 3").unwrap_or(vec![]);
        let mut exprs = exprs.iter();
        assert_eq!(into_string(&mut exprs), "(+ 1 (* 2 3))");
    }
}
