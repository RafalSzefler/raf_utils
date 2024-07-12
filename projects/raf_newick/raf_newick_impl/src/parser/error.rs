use crate::models::NewickGraphNewError;

#[derive(Debug)]
pub enum ParseError {
    InvalidContent,
    GraphError(NewickGraphNewError),
    IO(std::io::Error),
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
