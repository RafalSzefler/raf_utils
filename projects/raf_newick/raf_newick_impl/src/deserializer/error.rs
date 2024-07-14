#[derive(Debug)]
pub enum DeserializeError {
    InputError(std::io::Error),
}

impl From<std::io::Error> for DeserializeError {
    fn from(value: std::io::Error) -> Self {
        Self::InputError(value)
    }
}
