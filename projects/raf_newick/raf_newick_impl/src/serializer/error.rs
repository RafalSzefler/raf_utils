#[derive(Debug)]
pub enum SerializeError {
    IO(std::io::Error),
}

impl From<std::io::Error> for SerializeError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}
