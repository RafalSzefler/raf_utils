use std::{collections::{hash_map::Entry, HashMap}, io::Write};

use crate::models::{NewickGraph, NewickNode, NewickWeight};

use super::{SerializeError, SerializerOk};

pub(super) struct InternalSerializer<'a, TWrite: Write> {
    graph: &'a NewickGraph,
    output: &'a mut TWrite,
    written_bytes: usize,
    reticulation_id: u32,
    reticulation_map: HashMap<NewickNode, u32>,
}

impl<'a, TWrite: Write> InternalSerializer<'a, TWrite> {
    pub(super) fn new(
        graph: &'a NewickGraph,
        output: &'a mut TWrite) -> InternalSerializer<'a, TWrite>
    {
        Self {
            graph: graph,
            output: output,
            written_bytes: 0,
            reticulation_id: 0,
            reticulation_map: HashMap::new(),
        }
    }
    
    pub(super) fn serialize(mut self)
        -> Result<SerializerOk, SerializeError>
    {
        let root = self.graph.root_node();
        self.serialize_node(root)?;
        self.output.write_all(b";")?;
        self.output.flush()?;
        Ok(SerializerOk { written_bytes: self.written_bytes })
    }

    fn serialize_node(&mut self, node: NewickNode)
        -> Result<(), SerializeError>
    {
        macro_rules! _ser {
            ( $arr: expr ) => {
                {
                    let arrow = { $arr };
                    self.serialize_node(arrow.target())?;
                    if let Some(weight) = arrow.weight() {
                        self.serialize_weight(weight)?;
                    }
                }
            }
        }

        let is_reticulation = self.graph.get_incoming_arrows(node).len() > 1;

        let mut should_write_out = true;
        let reticulation_id = if is_reticulation {
            match self.reticulation_map.entry(node) {
                Entry::Occupied(entry) => {
                    should_write_out = false;
                    *entry.get()
                },
                Entry::Vacant(entry) => {
                    self.reticulation_id += 1;
                    let id = self.reticulation_id;
                    *entry.insert(id)
                },
            }
        } else {
            0
        };

        if should_write_out {
            let out = self.graph.get_outgoing_arrows(node);
            if !out.is_empty() {
                self.output.write_all(b"(")?;

                let mut iter = out.iter();
                _ser!(iter.next().unwrap());
                for arr in iter {
                    self.output.write_all(b",")?;
                    _ser!(arr);
                }
                self.output.write_all(b")")?;
            }
        }

        self.serialize_label(node)?;

        if is_reticulation {
            self.output.write_all(b"#")?;
            if let Some(reticulation_type) = self.graph.get_reticulation_type(node) {
                self.serialize_str(reticulation_type.as_str())?;
            }
            self.serialize_u32(reticulation_id)?;
        }

        return Ok(());
    }

    fn serialize_label(&mut self, node: NewickNode)
        -> Result<(), SerializeError>
    {
        if let Some(name) = self.graph.get_node_name(node) {
            self.serialize_str(name.as_str())?;
        }

        Ok(())
    }
    
    fn serialize_weight(&mut self, weight: NewickWeight)
        -> Result<(), SerializeError>
    {
        self.output.write_all(b":")?;
        self.serialize_u32(weight.integer_part())?;
        self.output.write_all(b".")?;
        self.serialize_u32(weight.fractional_part())?;
        Ok(())
    }

    #[allow(
        clippy::cast_lossless,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss)]
    fn serialize_u32(&mut self, mut value: u32)
        -> Result<(), SerializeError>
    {
        const MAX_LENGTH: usize = 10;
        let value_length = ((value as f64).log10().floor() as usize) + 1;
        assert!(value_length <= MAX_LENGTH, "u32 with base10 length longer than {MAX_LENGTH}?");
        let mut buffer = [0u8; MAX_LENGTH];

        for i in 1..=value_length {
            buffer[value_length - i] = b'0' + ((value % 10) as u8);
            value /= 10;
        }

        self.output.write_all(&buffer[0..value_length])?;
        Ok(())
    }

    fn serialize_str(&mut self, value: &str)
        -> Result<(), SerializeError>
    {
        if value.is_empty() {
            return Ok(());
        }

        if value.len() > 64 {
            return self.serialize_quoted_str(value);
        }

        for chr in value.chars() {
            if chr == '"' {
                return self.serialize_quoted_str(value);
            }
            if chr.is_whitespace() {
                return self.serialize_quoted_str(value);
            }
        }

        self.output.write_all(value.as_bytes())?;
        Ok(())
    }
    
    fn serialize_quoted_str(&mut self, value: &str)
        -> Result<(), SerializeError>
    {
        self.output.write_all(b"\"")?;
        let mut chr_buffer = [0u8; 4];
        for chr in value.chars() {
            if chr == '"' {
                self.output.write_all(b"\"\"")?;
                continue;
            }

            let len = chr.len_utf8();
            chr.encode_utf8(&mut chr_buffer);
            self.output.write_all(&chr_buffer[0..len])?;
            continue;
        }
        self.output.write_all(b"\"")?;
        Ok(())
    }
}
