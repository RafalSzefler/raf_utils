#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss)]

use std::{collections::HashSet, io::Write};

use crate::{
    common::{
        special_chars,
        BANG,
        COLON,
        COMMA,
        DOT,
        LEFT_BRACKET,
        QUOTE,
        RIGHT_BRACKET,
        SEMICOLON},
    models::{
        NewickGraph, NewickName, NewickNodeId, NewickReticulation, NewickReticulationKind, NewickWeight}};

use super::{SerializeError, SerializeOk};

pub struct Serializer<'a, TWrite: Write> {
    output: &'a mut TWrite,
    graph: &'a NewickGraph,
    written_bytes: usize,
    seen_reticulations: HashSet<NewickNodeId>,
}

impl<'a, TWrite: Write> Serializer<'a, TWrite> {
    pub fn new(output: &'a mut TWrite, graph: &'a NewickGraph) -> Self {
        Self {
            output: output,
            graph: graph,
            written_bytes: 0,
            seen_reticulations: HashSet::new(),
        }
    }

    /// Writes graph to the underlying stream.
    /// 
    /// # Errors
    /// * [`SerializeError::InvalidInput`] if graph is inconsistent
    /// * [`SerializeError::OutputError`] if couldn't write to underlying stream
    pub fn serialize(mut self) -> Result<SerializeOk, SerializeError> {
        let root = self.graph.root();
        self.serialize_node(root)?;
        self.write_char(SEMICOLON)?;
        Ok(SerializeOk { written_bytes: self.written_bytes })
    }

    fn serialize_node(&mut self, node_id: NewickNodeId)
        -> Result<(), SerializeError>
    {
        let Some(node) = self.graph.get_node_by_id(node_id) else {
            return Err(SerializeError::invalid("Graph has invalid nodes."));
        };

        let ret_data = node.reticulation();
        if ret_data.is_some() {
            if self.seen_reticulations.insert(node_id) {
                self.serialize_children(node_id)?;
            }
        } else {
            self.serialize_children(node_id)?;
        }

        let name = node.name().as_str();
        if !name.is_empty() {
            self.serialize_str(name)?;
        }

        if let Some(weight) = node.weight() {
            self.serialize_weight(weight)?;
        }

        if let Some(ret) = ret_data {
            self.serialize_reticulation(ret)?;
        }

        Ok(())
    }

    fn serialize_children(&mut self, node_id: NewickNodeId)
        -> Result<(), SerializeError>
    {
        let children = self.graph.get_children(node_id);
        if children.is_empty() {
            return Ok(());
        }

        self.write_char(LEFT_BRACKET)?;
        let mut iter = children.iter();
        self.serialize_node(*iter.next().unwrap())?;
        for child_id in iter {
            self.write_char(COMMA)?;
            self.serialize_node(*child_id)?;
        }
        self.write_char(RIGHT_BRACKET)?;
        Ok(())
    }

    fn serialize_str(&mut self, text: &str) -> Result<(), SerializeError> {
        const MAX_LEN: usize = {
            use crate::common::min;
            let name_max = NewickName::max_len();
            let ret_max = NewickReticulationKind::max_len();
            min(100, min(name_max, ret_max))
        };

        if text.is_empty() {
            return Ok(());
        }

        if text.len() > MAX_LEN {
            return self.serialize_quoted_str(text);
        }

        let special_chars = special_chars();
        for chr in text.chars() {
            if chr.is_whitespace() || special_chars.contains(&chr) {
                return self.serialize_quoted_str(text);
            }
        }

        self.write(text)?;
        Ok(())
    }

    fn serialize_quoted_str(&mut self, text: &str) -> Result<(), SerializeError> {
        self.write_char(QUOTE)?;

        for char in text.chars() {
            match char {
                QUOTE => {
                    self.write_char(QUOTE)?;
                    self.write_char(QUOTE)?;
                },
                _ => {
                    self.write_char(char)?;
                }
            }
        }

        self.write_char(QUOTE)?;
        Ok(())
    }
    
    fn serialize_weight(&mut self, weight: NewickWeight) -> Result<(), SerializeError> {
        self.write_char(COLON)?;
        self.serialize_u32(weight.integral_part())?;
        self.write_char(DOT)?;
        self.serialize_u32(weight.fractional_part())?;
        Ok(())
    }

    fn serialize_reticulation(&mut self, ret: &NewickReticulation) -> Result<(), SerializeError> {
        self.write_char(BANG)?;
        let kind = ret.kind().as_str();
        if !kind.is_empty() {
            self.serialize_str(kind)?;
        }
        self.serialize_u32(ret.id())?;
        Ok(())
    }

    fn serialize_u32(&mut self, value: u32) -> Result<(), SerializeError> {
        const MAX_LEN: usize = 11;
        if value == 0 {
            self.write("0")?;
            return Ok(());
        }

        let decimal_len = ((value as f64).log10() as usize) + 1;
        let mut buffer = [0u8; MAX_LEN];
        let mut tmp = value;
        for i in 0..decimal_len {
            buffer[decimal_len-i-1] = b'0' + (tmp % 10) as u8;
            tmp /= 10;
        }

        let slice = unsafe {
            core::str::from_utf8_unchecked(&buffer[0..decimal_len])
        };
        self.write(slice)?;
        Ok(())
    }

    fn write_char(&mut self, char: char) -> Result<(), SerializeError> {
        let mut buffer = [0u8; 4];
        let txt = char.encode_utf8(&mut buffer);
        self.write(txt)
    }

    #[inline(always)]
    fn write(&mut self, txt: &str) -> Result<(), SerializeError> {
        self.output.write_all(txt.as_bytes())?;
        self.written_bytes += txt.len();
        Ok(())
    }
}
