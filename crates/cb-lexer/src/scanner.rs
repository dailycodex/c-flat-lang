use super::{Span, Token, TokenDebug};
use std::{iter::Peekable, str::Chars};
type Stream<'a> = Peekable<Chars<'a>>;

pub struct Scanner<'a> {
    stream: Stream<'a>,
    span: Span,
    current: Option<char>,
    previous: Option<char>,
    token_debug: TokenDebug,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str, token_debug: TokenDebug) -> Self {
        Self {
            stream: src.chars().peekable(),
            span: 0..0,
            current: None,
            previous: None,
            token_debug,
        }
    }
    fn advance(&mut self) {
        match self.current {
            Some(c) => {
                self.span.end += c.to_string().as_bytes().len();
            }
            _ => {
                let Some(c) = self.previous else {
                    self.span.end += 1;
                    return;
                };
                self.span.end += c.to_string().as_bytes().len();
            }
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.previous = self.current;
        self.current = self.stream.next();
        self.advance();
        self.current
    }

    fn next_if(&mut self, func: impl FnOnce(&char) -> bool) -> Option<char> {
        match (self.current, self.stream.next_if(func)) {
            (Some(current), next @ Some(_)) => {
                self.previous = Some(current);
                self.current = next;
                self.advance();
            }
            (_, next @ Some(_)) => self.current = next,
            _ => return None,
        }
        self.current
    }

    fn matched(&mut self, check: char) -> bool {
        match self.peek_char() {
            Some(c) => c == &check,
            _ => false,
        }
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.stream.peek()
    }

    fn reset_span(&mut self) {
        self.span.start = self.span.end;
    }

    fn span(&mut self) -> Span {
        let span = self.span.clone();
        self.reset_span();
        span
    }

    fn number(&mut self) -> (Token, Span) {
        let mut number = self.current.unwrap().to_string();
        while let Some(ch) = self.next_if(|c| c.is_ascii_digit() || c == &'_' || c == &'.') {
            number.push(ch);
        }
        let span = self.span();
        let token = if number.contains('.') {
            Token::Float(number)
        } else {
            Token::Int(number)
        };
        (token, span)
    }

    fn id(&mut self) -> (Token, Span) {
        let mut ident = self.current.unwrap().to_string();
        while let Some(ch) = self.next_if(|c| c.is_ascii_alphanumeric() || c == &'_') {
            ident.push(ch);
        }
        let span = self.span();
        let token = Token::lookup(&ident).map_or(Token::Id(ident), |i| i);
        (token, span)
    }

    fn op_token(&mut self, op: &str) -> (Token, Span) {
        debug_assert!(!op.is_empty());
        for _ in 0..op.chars().count().saturating_sub(1) {
            self.next_char();
        }
        (Token::Op(op.into()), self.span())
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = (Token, Span);
    fn next(&mut self) -> Option<Self::Item> {
        let Some(ch) = self.next_char() else {
            let token = Some((Token::Eof, self.span()));
            if let TokenDebug::True = self.token_debug {
                let t = &token.clone().unwrap();
                eprintln!("{t:?}");
            }
            return token;
        };
        let token = match ch {
            num if num.is_ascii_digit() => Some(self.number()),
            ident if ident.is_ascii_alphabetic() => Some(self.id()),
            '-' if self.matched('>') => Some(self.op_token("->")),
            '=' if self.matched('=') => Some(self.op_token("==")),
            '>' if self.matched('=') => Some(self.op_token(">=")),
            '<' if self.matched('=') => Some(self.op_token("<=")),
            '!' if self.matched('=') => Some(self.op_token("!=")),
            '!' => Some(self.op_token("!")),
            '>' => Some(self.op_token(">")),
            '<' => Some(self.op_token("<")),
            '+' => Some(self.op_token("+")),
            '-' => Some(self.op_token("-")),
            '*' => Some(self.op_token("*")),
            '/' => Some(self.op_token("/")),
            '=' => Some(self.op_token("=")),
            ':' => Some(self.op_token(":")),
            ';' => Some(self.op_token(";")),
            ',' => Some(self.op_token(",")),
            '(' => Some(self.op_token("(")),
            ')' => Some(self.op_token(")")),
            '[' => Some(self.op_token("[")),
            ']' => Some(self.op_token("]")),
            '{' => Some(self.op_token("{")),
            '}' => Some(self.op_token("}")),
            'λ' => Some(self.op_token("λ")),
            ' ' | '\n' => {
                self.reset_span();
                self.next()
            }
            _ => Some((Token::Error(ch.into()), self.span())),
        };
        if let TokenDebug::True = self.token_debug {
            let t = &token.clone().unwrap_or((Token::Eof, self.span()));
            eprintln!("{t:?}");
        }
        token
    }
}
