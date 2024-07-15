use std::{fmt::Write, io::Read, mem::MaybeUninit};


use crate::{
    ast::{
        NewickGraphBuilder, NewickName, NewickNodeId, NewickReticulation, NewickReticulationKind, NewickWeight, OptionalNewickReticulation, OptionalNewickWeight},
    common::{
        special_chars, BANG, COLON, COMMA, DOT, LEFT_BRACKET, QUOTE, RIGHT_BRACKET, SEMICOLON}};

use super::{DeserializeError, DeserializeOk};

pub(crate) struct Deserializer<'a, TRead: Read> {
    input: &'a mut TRead,
    read_bytes: usize,
    read_chars: usize,
    current: char,
    builder: NewickGraphBuilder,
}

impl<'a, TRead: Read> Deserializer<'a, TRead> {
    pub fn new(input: &'a mut TRead) -> Self {
        Self {
            input: input,
            read_bytes: 0,
            read_chars: 0,
            current: ' ',
            builder: NewickGraphBuilder::default(),
        }
    }
    
    pub fn deserialize(mut self) -> Result<DeserializeOk, DeserializeError> {
        self.read_graph()?;
        let graph = self.builder.build()?;
        Ok(DeserializeOk {
            graph: graph,
            read_bytes: self.read_bytes
        })
    }

    fn read_graph(&mut self) -> Result<(), DeserializeError> {
        self.read_node()?;
        self.forward_whitespace()?;
        if self.current != SEMICOLON {
            let msg = format!("[char: {}] Expected '{}'.", self.read_chars, SEMICOLON);
            return Err(DeserializeError::FormatError(msg));
        }
        Ok(())
    }

    fn read_node(&mut self) -> Result<NewickNodeId, DeserializeError> {
        let children = self.read_children()?;
        let name = self.read_name()?;
        let weight = self.read_weight()?;
        let reticulation = self.read_reticulation()?;
        let new_node_id = self.builder.add_node(
            name,
            weight,
            reticulation,
            &children);
        Ok(new_node_id)
    }

    fn read_children(&mut self) -> Result<Vec<NewickNodeId>, DeserializeError> {
        self.forward_whitespace()?;
        if self.current != LEFT_BRACKET {
            return Ok(Vec::new());
        }

        let mut result = Vec::with_capacity(2);
        self.read_char()?;
        let node_id = self.read_node()?;
        result.push(node_id);
        loop {
            self.forward_whitespace()?;
            if self.current == COMMA {
                self.read_char()?;
                let node_id = self.read_node()?;
                result.push(node_id);
                continue;
            }

            if self.current == RIGHT_BRACKET {
                self.read_char()?;
                break;
            }

            let msg = format!("[char: {}] Expected '{}' or '{}'. Got: '{}'.", self.read_chars, COMMA, RIGHT_BRACKET, self.current);
            return Err(DeserializeError::FormatError(msg));
        }

        Ok(result)
    }

    fn read_name(&mut self) -> Result<NewickName, DeserializeError> {
        const MAX_LEN: usize = NewickName::max_len();

        self.forward_whitespace()?;
        if self.current == QUOTE {
            let text = self.read_str(MAX_LEN)?;
            let name = unsafe { NewickName::new_unchecked(text.as_str()) };
            return Ok(name);
        }

        let specials = special_chars();
        if specials.contains(&self.current) {
            return Ok(NewickName::default());
        }

        let text = self.read_str(MAX_LEN)?;
        let name = unsafe { NewickName::new_unchecked(text.as_str()) };
        return Ok(name);
    }

    fn read_weight(&mut self) -> Result<OptionalNewickWeight, DeserializeError> {
        self.forward_whitespace()?;
        if self.current != COLON {
            return Ok(OptionalNewickWeight::none());
        }

        self.read_char()?;
        let integral = self.read_u32()?;
        if self.current != DOT {
            let msg = format!("[char: {}] Expected '{}'. Got: '{}'.", self.read_chars, DOT, self.current);
            return Err(DeserializeError::FormatError(msg));
        }
        self.read_char()?;
        let fractional = self.read_u32()?;
        let weight = match NewickWeight::new(integral, fractional) {
            Ok(val) => val,
            Err(err) => {
                let msg = format!("[char: {}] Weight construction error: {:?}.", self.read_chars, err);
                return Err(DeserializeError::FormatError(msg));
            },
        };
        Ok(OptionalNewickWeight::some(weight))
    }

    fn read_reticulation(&mut self) -> Result<OptionalNewickReticulation, DeserializeError> {
        self.forward_whitespace()?;
        if self.current != BANG {
            return Ok(OptionalNewickReticulation::none());
        }

        self.read_char()?;
        let kind = if self.current.is_alphabetic() {
            let mut text = String::with_capacity(8);
            while self.current.is_alphabetic() {
                match text.write_char(self.current) {
                    Ok(_) => {},
                    Err(err) => {
                        let msg = format!("[char: {}] Error on temporary text write: {:?}.", self.read_chars, err);
                        return Err(DeserializeError::FormatError(msg));
                    },
                }
                if text.len() > NewickReticulationKind::max_len() {
                    let msg = format!("[char: {}] Max reticulation kind size exceeded.", self.read_chars);
                    return Err(DeserializeError::FormatError(msg));
                }
                self.read_char()?;
            }
            match NewickReticulationKind::new(text.as_str()) {
                Ok(val) => val,
                Err(err) => {
                    let msg = format!("[char: {}] Error on reticulation kind construction: {:?}.", self.read_chars, err);
                    return Err(DeserializeError::FormatError(msg));
                }
            }
        } else {
            NewickReticulationKind::default()
        };

        self.forward_whitespace()?;

        let id = self.read_u32()?;
        let ret = match NewickReticulation::new(id, kind) {
            Ok(val) => val,
            Err(err) => {
                let msg = format!("[char: {}] NewickReticulation construction error: {:?}.", self.read_chars, err);
                return Err(DeserializeError::FormatError(msg));
            },
        };
        Ok(OptionalNewickReticulation::some(ret))
    }

    fn read_u32(&mut self) -> Result<u32, DeserializeError> {
        let mut result = 0u32;
        if !self.current.is_ascii_digit() {
            let msg = format!("[char: {}] Expected digit, got: '{}'.", self.read_chars, self.current);
            return Err(DeserializeError::FormatError(msg));
        }

        while self.current.is_ascii_digit() {
            result = if let Some(val) = result.checked_mul(10) { val } else {
                let msg = format!("[char: {}] u32 overflow.", self.read_chars);
                return Err(DeserializeError::FormatError(msg));
            };
            let Some(u32_val) = self.current.to_digit(10) else {
                let msg = format!("[char: {}] invalid digit.", self.read_chars);
                return Err(DeserializeError::FormatError(msg));
            };
            result = if let Some(val) = result.checked_add(u32_val)  { val } else {
                let msg = format!("[char: {}] u32 overflow.", self.read_chars);
                return Err(DeserializeError::FormatError(msg));
            };
            self.read_char()?;
        }

        Ok(result)
    }

    fn read_str(&mut self, max_len: usize) -> Result<String, DeserializeError> {
        if self.current == QUOTE {
            self.read_char()?;
            return self.read_quoted_str(max_len);
        }

        return self.read_unquoted_str(max_len);
    }

    fn read_quoted_str(&mut self, max_len: usize) -> Result<String, DeserializeError> {
        let mut result = String::with_capacity(4);
        macro_rules! write {
            ( $chr: expr ) => {
                match result.write_char(self.current) {
                    Ok(_) => { },
                    Err(err) => {
                        let msg = format!("[char: {}] Error on temporary text write: {:?}.", self.read_chars, err);
                        return Err(DeserializeError::FormatError(msg));
                    }
                }
            };
        }

        for _ in 0..max_len {
            if self.current == QUOTE {
                self.read_char()?;
                if self.current != QUOTE {
                    return Ok(result);
                }
                write!(QUOTE);
                continue;
            }

            write!(self.current);
            self.read_char()?;
        }

        let msg = format!("[char: {}] Maximum length of string exceeded.", self.read_chars);
        return Err(DeserializeError::FormatError(msg));
    }

    fn read_unquoted_str(&mut self, max_len: usize) -> Result<String, DeserializeError> {
        let mut result = String::with_capacity(4);
        macro_rules! write {
            ( $chr: expr ) => {
                match result.write_char(self.current) {
                    Ok(_) => { },
                    Err(err) => {
                        let msg = format!("[char: {}] Error on temporary text write: {:?}.", self.read_chars, err);
                        return Err(DeserializeError::FormatError(msg));
                    }
                }
            };
        }

        let specials = special_chars();
        for _ in 0..max_len {
            if self.current.is_whitespace() || specials.contains(&self.current) {
                return Ok(result);
            }
            write!(self.current);
            self.read_char()?;
        }

        let msg = format!("[char: {}] Maximum length of string exceeded.", self.read_chars);
        return Err(DeserializeError::FormatError(msg));
    }

    fn forward_whitespace(&mut self) -> Result<(), DeserializeError> {
        while self.current.is_whitespace() {
            self.read_char()?;
        }
        Ok(())
    }

    fn read_char(&mut self) -> Result<(), DeserializeError> {
        const MAX_UTF8_CHAR_SIZE: usize = 4;
        let mut buffer = [0u8; MAX_UTF8_CHAR_SIZE];
        let mut offset = 1;

        let mut last_error = MaybeUninit::uninit();

        while offset < MAX_UTF8_CHAR_SIZE {
            self.input.read_exact(&mut buffer[(offset-1)..offset])?;
            match std::str::from_utf8(&buffer[0..offset]) {
                Ok(text) => {
                    self.read_bytes += offset;
                    self.read_chars += 1;
                    self.current = text.chars().next().unwrap();
                    return Ok(());
                },
                Err(err) => {
                    last_error.write(err);
                    offset += 1;
                },
            }
        }

        let err = unsafe { last_error.assume_init() };
        Err(err.into())
    }
}
