use core::hash::{Hash, Hasher};


use crate::hash_helpers::calculate_hash;

use super::to_byte_slice::ToByteSlice;


#[derive(Clone)]
pub(super) struct TemporaryString {
    raw_ptr: *const u8,
    length: u32,
    hash: u32,
}

unsafe impl Sync for TemporaryString { }
unsafe impl Send for TemporaryString { }


impl TemporaryString {
    #[inline(always)]
    pub(super) fn from_byte_slicable<T: ?Sized + ToByteSlice>(value: &T) -> Self {
        let bytes = value.to_slice();
        Self {
            raw_ptr: bytes.as_ptr(),
            length: bytes.len() as u32,
            hash: calculate_hash(bytes),
        }
    }

    #[inline(always)]
    pub(super) fn from_byte_slicable_with_hash<T: ?Sized + ToByteSlice>(value: &T, hash: u32) -> Self {
        let bytes = value.to_slice();
        Self {
            raw_ptr: bytes.as_ptr(),
            length: bytes.len() as u32,
            hash: hash,
        }
    }

    #[inline(always)]
    fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.raw_ptr, self.length as usize) }
    }
}

impl PartialEq for TemporaryString {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
            && self.as_slice() == other.as_slice()
    }
}

impl Eq for TemporaryString { }

impl Hash for TemporaryString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}
