use std::io::Read;

use super::{DeserializeError, DeserializeOk};

pub struct Deserializer<'a, TRead: Read> {
    input: &'a mut TRead,
}

impl<'a, TRead: Read> Deserializer<'a, TRead> {
    pub fn new(input: &'a mut TRead) -> Self {
        Self { input }
    }
    
    pub fn deserialize(self) -> Result<DeserializeOk, DeserializeError> {
        todo!()
    }
}
