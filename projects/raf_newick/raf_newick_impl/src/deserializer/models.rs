use std::io::Read;

use super::{DeserializeError, DeserializeOk};

pub(crate) struct Deserializer<'a, TRead: Read> {
    input: &'a mut TRead,
    read_bytes: usize,
}

impl<'a, TRead: Read> Deserializer<'a, TRead> {
    pub fn new(input: &'a mut TRead) -> Self {
        Self {
            input: input,
            read_bytes: 0
        }
    }
    
    pub fn deserialize(self) -> Result<DeserializeOk, DeserializeError> {
        todo!()
    }

    fn read_char(&mut self) -> Result<char, DeserializeError> {
        const MAX_UTF8_CHAR_SIZE: usize = 4;
        let mut buffer = [0u8; MAX_UTF8_CHAR_SIZE];
        let mut offset = 1;

        let mut last_error = None;

        while offset < MAX_UTF8_CHAR_SIZE {
            self.input.read_exact(&mut buffer[(offset-1)..offset])?;
            match std::str::from_utf8(&buffer[0..offset]) {
                Ok(text) => {
                    self.read_bytes += offset;
                    let char = text.chars().next().unwrap();
                    return Ok(char);
                },
                Err(err) => {
                    last_error = Some(err);
                    offset += 1;
                },
            }
        }

        Err(last_error.unwrap().into())
    }
}
