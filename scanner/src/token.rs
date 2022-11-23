use super::{Span, TokenType};

pub struct Token {
    span: Span,
    tok_type: TokenType,
}

impl Token {
    pub fn new(span: Span, tok_type: TokenType) -> Self {
        Self { span, tok_type }
    }

    pub fn tok_type(&self) -> &TokenType {
        &self.tok_type
    }
}
