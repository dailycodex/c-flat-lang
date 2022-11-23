use super::span::Pos;
use super::{KeyWord, Span, Token, TokenType};
use anyhow::{bail, Result};
use std::{iter::Peekable, str::Chars};
type Stream<'a> = Peekable<Chars<'a>>;

pub struct Scanner<'a> {
    filename: &'a str,
    stream: Stream<'a>,
    pos: Pos,
    current: Option<char>,
    previous: Option<char>,
}

impl<'a> Scanner<'a> {
    pub fn new(filename: &'a str, src: &'a str) -> Self {
        Self {
            filename,
            stream: src.chars().peekable(),
            pos: Pos::default(),
            current: None,
            previous: None,
        }
    }
    fn advance(&mut self) {
        match self.current {
            Some('\n') => self.pos.newline(),
            _ => self.pos.right_shift(),
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

    fn peek(&mut self) -> Option<&char> {
        self.stream.peek()
    }

    fn span(&self, start: Pos) -> Span {
        Span::new(self.filename, start, self.pos)
    }

    fn number(&mut self) -> Result<Token> {
        let mut number = self.current.unwrap().to_string();
        let start = self.pos;
        while let Some(ch) = self.next_if(|c| c.is_ascii_digit() || c == &'_') {
            number.push(ch);
        }
        let span = self.span(start);
        Ok(Token::new(span, TokenType::Int(number)))
    }

    fn id(&mut self) -> Result<Token> {
        let mut ident = self.current.unwrap().to_string();
        let start = self.pos;
        while let Some(ch) = self.next_if(|c| c.is_ascii_alphanumeric() || c == &'_') {
            ident.push(ch);
        }
        let span = self.span(start);
        let token_type = KeyWord::lookup(&ident).map_or(TokenType::Id(ident), TokenType::KeyWord);
        Ok(Token::new(span, token_type))
    }

    fn return_arrow(&mut self) -> Result<Token> {
        let _ = self.next_char();
        Ok(Token::new(self.span(self.pos), TokenType::RArrow))
    }

    fn eof(&mut self) -> Result<Token> {
        Ok(Token::new(self.span(self.pos), TokenType::Eof))
    }

    pub fn next(&mut self) -> Result<Token> {
        if let Some(ch) = self.next_char() {
            dbg!(ch);
            match ch {
                num if num.is_ascii_digit() => return self.number(),
                ident if ident.is_ascii_alphabetic() => return self.id(),
                '-' if self.peek() == Some(&'>') => return self.return_arrow(),
                '+' => return Ok(Token::new(self.span(self.pos), TokenType::Op(ch.into()))),
                '-' => return Ok(Token::new(self.span(self.pos), TokenType::Op(ch.into()))),
                '=' => return Ok(Token::new(self.span(self.pos), TokenType::Ctrl(ch))),
                ':' => return Ok(Token::new(self.span(self.pos), TokenType::Ctrl(ch))),
                ';' => return Ok(Token::new(self.span(self.pos), TokenType::Ctrl(ch))),
                ',' => return Ok(Token::new(self.span(self.pos), TokenType::Ctrl(ch))),
                '(' => return Ok(Token::new(self.span(self.pos), TokenType::Ctrl(ch))),
                ')' => return Ok(Token::new(self.span(self.pos), TokenType::Ctrl(ch))),
                '[' => return Ok(Token::new(self.span(self.pos), TokenType::Ctrl(ch))),
                ']' => return Ok(Token::new(self.span(self.pos), TokenType::Ctrl(ch))),
                '{' => return Ok(Token::new(self.span(self.pos), TokenType::Ctrl(ch))),
                '}' => return Ok(Token::new(self.span(self.pos), TokenType::Ctrl(ch))),
                ' ' | '\n' => return self.next(),
                _ => bail!("{}: unknown char '{}'", self.span(self.pos), ch),
            }
        }
        self.eof()
    }
}
