#[derive(Debug)]
pub enum SerializeError {
    InvalidInput(String),
    OutputError(std::io::Error),
}

impl SerializeError {
    pub(crate) fn invalid(text: impl Into<String>) -> Self {
        Self::InvalidInput(text.into())
    }
}

impl From<std::io::Error> for SerializeError {
    fn from(value: std::io::Error) -> Self {
        Self::OutputError(value)
    }
}
