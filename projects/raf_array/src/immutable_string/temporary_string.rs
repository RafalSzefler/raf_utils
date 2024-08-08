use core::hash::{Hash, Hasher};


use super::{hash_helpers::calculate_hash, to_byte_slice::ToByteSlice};


#[derive(Clone)]
pub(super) struct TemporaryString {
    raw_ptr: *const u8,
    length: usize,
    hash: u64,
}

unsafe impl Sync for TemporaryString { }
unsafe impl Send for TemporaryString { }


impl TemporaryString {
    #[inline(always)]
    pub(super) fn from_byte_slicable<T: ?Sized + ToByteSlice>(value: &T) -> Self {
        let bytes = value.to_slice();
        Self {
            raw_ptr: bytes.as_ptr(),
            length: bytes.len(),
            hash: calculate_hash(bytes),
        }
    }

    #[inline(always)]
    pub(super) fn from_byte_slicable_with_hash<T: ?Sized + ToByteSlice>(value: &T, hash: u64) -> Self {
        let bytes = value.to_slice();
        Self {
            raw_ptr: bytes.as_ptr(),
            length: bytes.len(),
            hash: hash,
        }
    }

    #[inline(always)]
    pub fn hash(&self) -> u64 {
        self.hash
    }

    #[inline(always)]
    fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.raw_ptr, self.length) }
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