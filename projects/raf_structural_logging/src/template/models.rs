use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex}};

use raf_array::{atomic_array::{StrongArray, StrongArrayBuilder}, immutable_string::ImmutableString};

use super::parser::parse_template_to_pieces;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum TemplatePiece {
    Empty,
    RawString(ImmutableString),
    Parameter(ImmutableString),
}

impl Default for TemplatePiece {
    fn default() -> Self { Self::Empty }
}

#[derive(Debug, Default, Clone)]
pub struct Template {
    raw: ImmutableString,
    pieces: StrongArray<TemplatePiece>,
}

impl PartialEq for Template {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl Eq for Template { }

impl core::hash::Hash for Template {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl Template {
    #[inline(always)]
    fn new(raw: ImmutableString, pieces: StrongArray<TemplatePiece>) -> Self {
        Self { raw, pieces }
    }

    #[inline(always)]
    pub fn as_immutable_string(&self) -> &ImmutableString { &self.raw }

    #[inline(always)]
    pub fn pieces(&self) -> &[TemplatePiece] { self.pieces.as_slice() }
}

#[derive(Default)]
pub struct TemplateBuilder {
    cache: Mutex<HashMap<ImmutableString, Template>>,
}

impl TemplateBuilder {
    pub fn create(&self, txt: &str) -> Template {
        let imm = ImmutableString::new(txt).unwrap();

        {
            let guard = self.cache.lock().unwrap();
            if let Some(tmpl) = guard.get(&imm) {
                return tmpl.clone();
            }
        }

        let pieces = parse_template_to_pieces(&imm);
        let strong_pieces = StrongArrayBuilder::default().build_from_clonable(&pieces).unwrap();
        let new_tmpl = Template::new(
            imm.clone(), 
            strong_pieces);

        {
            let mut guard = self.cache.lock().unwrap();
            if let Some(tmpl) = guard.get(&imm) {
                return tmpl.clone();
            }
            guard.insert(imm, new_tmpl.clone());
        }

        new_tmpl
    }
}


static LOGGER_NAME: LazyLock<ImmutableString>
    = LazyLock::new(|| {
        ImmutableString::new("logger_name").unwrap()
    });

#[inline(always)]
pub fn get_logger_name_key() -> &'static ImmutableString { &LOGGER_NAME }
