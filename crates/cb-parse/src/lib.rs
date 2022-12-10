use cb_lexer::{Scanner, Span, Token, TokenDebug};
use std::fmt;
use std::iter::Peekable;

type CResult<T> = Result<T, Box<dyn std::error::Error>>;

pub enum ParseDebug {
    True,
    False,
}

impl From<bool> for ParseDebug {
    fn from(b: bool) -> Self {
        if b {
            return Self::True;
        }
        Self::False
    }
}

pub fn parse(src: &str, scan_debug: TokenDebug, parse_debug: ParseDebug) -> CResult<Vec<Expr>> {
    let lexer = Scanner::new(src, scan_debug);
    let mut parser = Parser::new(lexer.peekable());
    let ast = parser.parse();
    if let ParseDebug::True = parse_debug {
        dbg!(&ast);
    }
    ast
}

#[derive(Debug)]
enum ParserError {
    BadToken(Option<(Token, Span)>),
    Expected(Token, (Token, Span)),
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadToken(Some((t, s))) => write!(f, "{}:{} {t:?}", s.start, s.end),
            Self::BadToken(None) => write!(f, "BadToken::None"),
            Self::Expected(expected, (found, s)) => write!(
                f,
                "{}:{} expected '{expected:?}' but found '{found:?}",
                s.start, s.end
            ),
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
    Grt,
    Les,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minus => write!(f, "-"),
            Self::Plus => write!(f, "+"),
            Self::Mult => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Grt => write!(f, ">"),
            Self::Les => write!(f, "<"),
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
            ">" => Ok(Self::Grt),
            "<" => Ok(Self::Les),
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
    Id(String),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{i}"),
            Self::Id(i) => write!(f, "{i}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Atom(Atom),
    Unary(Op, Box<Self>),
    Binary(Op, Box<Self>, Box<Self>),
    If(Box<Self>, Box<Self>),
    IfElse(Box<Self>, Box<Self>, Box<Self>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(i) => write!(f, "{i}"),
            Self::Unary(op, expr) => write!(f, "({op} {expr})"),
            Self::Binary(op, lhs, rhs) => write!(f, "({op} {lhs} {rhs})"),
            Self::If(c, b) => write!(f, "(if ({c}) ({b}))"),
            Self::IfElse(c, b1, b2) => write!(f, "(if ({c}) then ({b1}) else ({b2}))"),
        }
    }
}

struct Parser<'a> {
    lexer: Peekable<Scanner<'a>>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Peekable<Scanner<'a>>) -> Self {
        Self { lexer }
    }

    fn is_end(&mut self) -> bool {
        matches!(self.lexer.peek().unwrap(), (Token::Eof, _))
    }

    fn check(&mut self, expected: Token) -> bool {
        self.peek() == expected
    }

    fn consume(&mut self, expected: Token) -> CResult<Span> {
        if self.peek() == expected {
            return Ok(self.lexer.next().map(|(_, s)| s).unwrap());
        }
        let found = self.lexer.peek().unwrap().clone();
        Err(Box::new(ParserError::Expected(expected, found)))
    }

    fn peek(&mut self) -> Token {
        self.lexer
            .peek()
            .map(|(t, _)| t.clone())
            .unwrap_or(Token::Eof)
    }

    fn program(&mut self) -> CResult<Expr> {
        self.if_statement()
    }

    fn if_statement(&mut self) -> CResult<Expr> {
        if self.check(Token::KeyWord("if".into())) {
            let span = self.consume(Token::KeyWord("if".into()))?;
            let condition = self.expression(Precedence::None)?;
            self.consume(Token::Op("{".into()))?;
            let branch = self.if_statement()?;
            self.consume(Token::Op("}".into()))?;
            if self.check(Token::KeyWord("else".into())) {
                return self.if_else_statement(span, condition, branch);
            }
            let expr = Expr::If(Box::new(condition), Box::new(branch));
            return Ok(expr);
        }
        self.expression(Precedence::None)
    }

    fn if_else_statement(&mut self, _span: Span, condition: Expr, branch1: Expr) -> CResult<Expr> {
        self.consume(Token::KeyWord("else".into()))?;
        let branch2 = if self.check(Token::KeyWord("if".into())) {
            self.if_statement()?
        } else {
            self.consume(Token::Op("{".into()))?;
            let branch2 = self.if_statement()?;
            self.consume(Token::Op("}".into()))?;
            branch2
        };
        Ok(Expr::IfElse(
            Box::new(condition),
            Box::new(branch1),
            Box::new(branch2),
        ))
    }

    fn expression(&mut self, min_bp: Precedence) -> CResult<Expr> {
        let mut lhs = match self.lexer.next() {
            Some((Token::Int(a), _)) => Expr::Atom(Atom::Int(a.parse().unwrap())),
            Some((Token::Id(id), _)) => Expr::Atom(Atom::Id(id.into())),
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
            let Some((token, _)) = self.lexer.peek() else {
                    break;
                };
            let bp = Precedence::from(token.clone());
            let op = match Op::try_from(token.clone()) {
                Ok(o) => o,
                Err(_) => break,
            };
            if bp <= min_bp {
                break;
            }
            self.lexer.next();
            match bp {
                Precedence::Term | Precedence::Factor | Precedence::Comparison => {
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
    fn tparse(src: &str) -> Vec<Expr> {
        parse(src, TokenDebug::True, ParseDebug::True).unwrap_or(vec![])
    }

    fn into_string<E: fmt::Display>(i: &mut impl Iterator<Item = E>) -> String {
        i.next().map(|t| t.to_string()).unwrap_or("".into())
    }

    #[test]
    fn expressions() {
        let exprs = tparse("1");
        let mut exprs = exprs.iter();
        assert_eq!(into_string(&mut exprs), "1");
    }

    #[test]
    fn change_precedence() {
        let exprs = tparse("(1)");
        let mut exprs = exprs.iter();
        assert_eq!(into_string(&mut exprs), "1");
    }

    #[test]
    fn unary() {
        let exprs = tparse("-1");
        let mut exprs = exprs.iter();
        assert_eq!(into_string(&mut exprs), "(- 1)");
    }

    #[test]
    fn binary() {
        let exprs = tparse("1 + 1");
        let mut exprs = exprs.iter();
        assert_eq!(into_string(&mut exprs), "(+ 1 1)");

        let exprs = tparse("1 + 2 * 3");
        let mut exprs = exprs.iter();
        assert_eq!(into_string(&mut exprs), "(+ 1 (* 2 3))");

        let exprs = tparse("1 > 2");
        let mut exprs = exprs.iter();
        assert_eq!(into_string(&mut exprs), "(> 1 2)");
    }

    #[test]
    fn if_statement() {
        let exprs = tparse("if 1 > 3 { a + b }");
        let mut exprs = exprs.iter();
        assert_eq!(into_string(&mut exprs), "(if ((> 1 3)) ((+ a b)))");
    }

    #[test]
    fn if_else_statement() {
        let exprs = tparse("if x > y { x } else { y }");
        let mut exprs = exprs.iter();
        assert_eq!(into_string(&mut exprs), "(if ((> x y)) then (x) else (y))");

        let exprs = tparse("if x > y { x } else if x < y { y + y } else { y }");
        let mut exprs = exprs.iter();
        assert_eq!(
            into_string(&mut exprs),
            "(if ((> x y)) then (x) else ((if ((< x y)) then ((+ y y)) else (y))))"
        );
    }
}
