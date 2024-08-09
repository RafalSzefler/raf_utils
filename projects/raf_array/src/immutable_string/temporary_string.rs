use core::hash::{Hash, Hasher};

use super::to_temporary_string_params::ToTemporaryStringParams;

#[derive(Clone)]
pub(super) struct TemporaryString {
    raw_ptr: *const u8,
    length: u32,
    hash_value: u32,
}

unsafe impl Sync for TemporaryString { }
unsafe impl Send for TemporaryString { }


impl TemporaryString {
    #[allow(clippy::cast_possible_truncation)]
    #[inline(always)]
    pub(super) fn from_to_params<T: ?Sized + ToTemporaryStringParams>(params: &T) -> Self {
        let result = params.to_params();
        let slice = result.slice();
        Self {
            raw_ptr: slice.as_ptr(),
            length: slice.len() as u32,
            hash_value: result.hash(),
        }
    }

    #[inline(always)]
    pub(super) const fn hash_value(&self) -> u32 {
        self.hash_value
    }

    #[inline(always)]
    fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.raw_ptr, self.length as usize) }
    }
}

impl PartialEq for TemporaryString {
    fn eq(&self, other: &Self) -> bool {
        self.hash_value == other.hash_value
            && self.as_slice() == other.as_slice()
    }
}

impl Eq for TemporaryString { }

impl Hash for TemporaryString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash_value.hash(state);
    }
}
