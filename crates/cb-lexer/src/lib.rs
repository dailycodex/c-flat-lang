mod scanner;
#[cfg(test)]
mod test;
mod token;

pub type Span = std::ops::Range<usize>;
pub use crate::scanner::Scanner;
pub use crate::token::Token;

pub enum TokenDebug {
    True,
    False,
}

impl From<bool> for TokenDebug {
    fn from(b: bool) -> Self {
        if b {
            return Self::True;
        }
        Self::False
    }
}
