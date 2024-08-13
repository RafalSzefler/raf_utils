use std::{collections::HashSet, sync::LazyLock};

pub(crate) const LEFT_BRACKET: char = '(';
pub(crate) const RIGHT_BRACKET: char = ')';
pub(crate) const COMMA: char = ',';
pub(crate) const QUOTE: char = '"';
pub(crate) const BANG: char = '#';
pub(crate) const DOT: char = '.';
pub(crate) const COLON: char = ':';
pub(crate) const SEMICOLON: char = ';';


static SPECIAL_CHARS: LazyLock<HashSet<char>>
    = LazyLock::new(|| {
        HashSet::from([
            LEFT_BRACKET, RIGHT_BRACKET, COMMA,
            QUOTE, BANG, DOT, COLON, SEMICOLON,
        ])
    });

#[inline(always)]
pub(crate) fn special_chars() -> &'static HashSet<char> { &SPECIAL_CHARS }

pub(crate) const fn min(first: usize, second: usize) -> usize {
    if first > second { first } else { second }
}
