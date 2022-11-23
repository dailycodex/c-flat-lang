use super::Span;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}: unknown char '{1}'")]
    UnknowChar(Span, char),
}
