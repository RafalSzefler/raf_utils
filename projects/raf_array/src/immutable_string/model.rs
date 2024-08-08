use core::{
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    mem::ManuallyDrop,
};
use std::ops::Deref;

use crate::atomic_array::StrongArray;

use super::{
    cache::CACHE, errors::NewImmutableStringError, temporary_string::TemporaryString, weak_string::WeakString, StringId
};

#[derive(Clone)]
pub struct ImmutableString {
    array: ManuallyDrop<StrongArray<u8>>,
    hash: u64,
}


impl ImmutableString {
    #[inline(always)]
    pub const fn max_byte_length() -> usize {
        StrongArray::<u8>::max_byte_length()
    }

    /// Creates a [`ImmutableString`] out of `&str`. If there already
    /// exists an [`ImmutableString`] representing the same `&str` then
    /// no allocation is performed and the cached value is returned.
    /// Otherwise the function allocates new [`ImmutableString`] and
    /// copies the content of `&str` into it.
    /// 
    /// # Errors
    /// * [`NewImmutableStringError::MaxLengthExceeded`] if total byte length
    ///   exceeds [`ImmutableString::max_byte_length()`].
    /// * [`NewImmutableStringError::AllocationError`] if couldn't allocate
    ///   underlying memory.
    pub fn new(text: &str) -> Result<Self, NewImmutableStringError> {
        if text.is_empty() {
            return Ok(Self::default());
        }

        let tmp = TemporaryString::from_byte_slicable(text);
        let hash = tmp.hash();

        if let Some(weak) = CACHE.get(&tmp) {
            if let Ok(strong) = weak.array().upgrade() {
                return Ok(Self::from_strong(strong, hash));
            }
        }

        let new_strong = StrongArray::<u8>::copy_slice(text.as_bytes())?;
        let new_tmp = TemporaryString::from_byte_slicable_with_hash(&new_strong, hash);
        let weak = WeakString::from_weak_array(new_strong.downgrade());

        CACHE.set(&new_tmp, weak);

        Ok(Self::from_strong(new_strong, new_tmp.hash()))
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    #[inline(always)]
    pub fn id(&self) -> StringId {
        StringId::from_array_id(self.array.id())
    }

    /// Returns the internal [`StrongArray`] by bumping its strong
    /// ref counter.
    /// 
    /// # Safety
    /// [`ImmutableString`] strongly depends on owning the memory. It
    /// is important that the last reference is owned by [`ImmutableString`],
    /// not by [`StrongArray`], because [`ImmutableString`] does additional
    /// work on when the last reference is dropped. It is up to caller to
    /// ensure that, that's why the method is unsafe.
    #[inline(always)]
    pub unsafe fn as_strong_array(imm: &Self) -> StrongArray<u8> {
        imm.array.deref().clone()
    }

    /// Retruns the number of alive strong references.
    #[inline(always)]
    pub fn strong_count(imm: &Self) -> u32 {
        imm.array.strong_count()
    }

    #[inline(always)]
    fn from_strong(strong: StrongArray<u8>, hash: u64) -> Self {
        Self {
            array: ManuallyDrop::new(strong),
            hash: hash,
        }
    }
}

impl Default for ImmutableString {
    fn default() -> Self {
        Self {
            array: ManuallyDrop::new(StrongArray::default()),
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

impl AsRef<[u8]> for ImmutableString {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.array.as_slice()
    }
}

impl Display for ImmutableString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl Debug for ImmutableString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImmutableString")
            .field("text", &self.as_str())
            .field("hash", &self.hash)
            .field("id", &self.id())
            .finish()
    }
}

impl Drop for ImmutableString {
    fn drop(&mut self) {
        let array = unsafe { ManuallyDrop::take(&mut self.array) };
        if let Some(unique) = array.release() {
            let key = TemporaryString::from_byte_slicable(&unique);
            CACHE.remove(&key);
        }
    }
}
