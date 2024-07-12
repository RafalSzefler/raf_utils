use crate::models::NewickGraphNewError;

#[derive(Debug)]
pub enum ParseError {
    InvalidContent(String),
    GraphError(NewickGraphNewError),
    IO(std::io::Error),
    UTF8(std::str::Utf8Error),
}

impl From<NewickGraphNewError> for ParseError {
    fn from(value: NewickGraphNewError) -> Self {
        Self::GraphError(value)
    }
}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<std::str::Utf8Error> for ParseError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::UTF8(value)
    }
}
