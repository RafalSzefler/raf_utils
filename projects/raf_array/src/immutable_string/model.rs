use core::{
    fmt::{Display, Formatter},
    hash::{Hash, Hasher}
};

use crate::atomic_array::StrongArray;

#[derive(Clone)]
pub struct ImmutableString {
    array: StrongArray<u8>,
    hash: u64,
}

impl ImmutableString {
    #[inline(always)]
    pub const fn get_max_length() -> usize {
        StrongArray::<u8>::max_byte_length()
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl Default for ImmutableString {
    fn default() -> Self {
        Self {
            array: StrongArray::default(),
            hash: 0,
        }
    }
}

impl PartialEq for ImmutableString {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
            && self.array == other.array
    }
}

impl Eq for ImmutableString { }

impl Hash for ImmutableString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl AsRef<str> for ImmutableString {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        let slice = self.array.as_slice();
        unsafe { core::str::from_utf8_unchecked(slice) }
    }
}

impl Display for ImmutableString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}