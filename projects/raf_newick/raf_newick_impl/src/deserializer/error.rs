use crate::ast::InvalidGraphError;

#[derive(Debug)]
pub enum DeserializeError {
    FormatError(String),
    GraphError(InvalidGraphError),
    InputError(std::io::Error),
    Utf8(std::str::Utf8Error),
}

impl From<std::io::Error> for DeserializeError {
    fn from(value: std::io::Error) -> Self {
        Self::InputError(value)
    }
}

impl From<std::str::Utf8Error> for DeserializeError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::Utf8(value)
    }
}

impl From<InvalidGraphError> for DeserializeError {
    fn from(value: InvalidGraphError) -> Self {
        Self::GraphError(value)
    }
}
