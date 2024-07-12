use std::io::Read;

use crate::models::NewickGraph;

use super::{ParseError, ParseOk};

pub(super) struct InternalDeserializer<'a, TRead: Read> {
    input: &'a mut TRead,
    read_bytes: usize,
}

impl<'a, TRead: Read> InternalDeserializer<'a, TRead> {
    pub(super) fn new(
        input: &'a mut TRead) -> Self
    {
        Self {
            input: input,
            read_bytes: 0,
        }
    }

    pub(super) fn parse(mut self) -> Result<ParseOk, ParseError> {
        let graph = self.read_graph()?;
        Ok(ParseOk {
            graph: graph,
            read_bytes: self.read_bytes
        })
    }

    fn read_next_char(&mut self) -> Result<char, ParseError> {
        let mut char_buffer = [0u8; 4];

        let mut start = 0usize;
        let mut read = 0;

        let mut last_error = None;

        while start < 4 {
            self.input.read_exact(&mut char_buffer[start..=start])?;
            read += 1;

            match core::str::from_utf8(&char_buffer[0..read]) {
                Ok(text) => {
                    self.read_bytes += read;
                    return Ok(text.chars().next().unwrap());
                },
                Err(err) => {
                    last_error = Some(err);
                }
            };

            start += 1;
        }

        Err(last_error.unwrap().into())
    }
    
    fn read_graph(&mut self) -> Result<NewickGraph, ParseError> {
        todo!()
    }
}
