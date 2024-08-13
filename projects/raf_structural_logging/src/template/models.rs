use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex}};

use raf_array::{atomic_array::{StrongArray, StrongArrayBuilder}, immutable_string::ImmutableString};

use super::parser::parse_template_to_pieces;

/// Represents two different pieces of template: raw string and parameter.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum TemplatePiece {
    RawString(ImmutableString),
    Parameter(ImmutableString),
}

/// Represents textual template. Template should be of the form
/// `"This is {val} value"` which the engine splits into three pieces:
/// `RawString("This is")` -> `Parameter("val")` -> `RawString(" value")`.
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

/// Represents builder for [`Template`] out of string. Will cache already
/// constructed templates.
#[derive(Default)]
pub struct TemplateBuilder {
    cache: Mutex<HashMap<ImmutableString, Template>>,
}

impl TemplateBuilder {
    pub fn create(&self, imm: &ImmutableString) -> Template {
        {
            let guard = self.cache.lock().unwrap();
            if let Some(tmpl) = guard.get(imm) {
                return tmpl.clone();
            }
        }

        let pieces = parse_template_to_pieces(imm);
        let strong_pieces = StrongArrayBuilder::default().build_from_clonable(&pieces).unwrap();
        let new_tmpl = Template::new(
            imm.clone(), 
            strong_pieces);

        {
            let mut guard = self.cache.lock().unwrap();
            if let Some(tmpl) = guard.get(imm) {
                return tmpl.clone();
            }
            guard.insert(imm.clone(), new_tmpl.clone());
        }

        new_tmpl
    }

    pub fn create_from_str(&self, txt: &str) -> Template {
        let imm = ImmutableString::new(txt).unwrap();
        self.create(&imm)
    }
}


static LOGGER_NAME: LazyLock<ImmutableString>
    = LazyLock::new(|| {
        ImmutableString::new("logger_name").unwrap()
    });

#[inline(always)]
pub(crate) fn get_logger_name_key() -> &'static ImmutableString { &LOGGER_NAME }
