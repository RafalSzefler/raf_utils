use core::{
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    mem::ManuallyDrop,
};
use std::ops::Deref;

use crate::atomic_array::{StrongArray, StrongArrayBuilder};

use super::{
    cache::CACHE,
    NewImmutableStringError,
    temporary_string::TemporaryString,
    StringId,
};

/// Represents an immutable string, in its essence similar to `Arc<[u8]>`,
/// except more compact, slightly more efficient and with better API.
#[derive(Clone)]
#[repr(transparent)]
pub struct ImmutableString {
    array: ManuallyDrop<StrongArray<u8>>,
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

        if text.len() > Self::max_byte_length() {
            return Err(NewImmutableStringError::MaxLengthExceeded);
        }

        let tmp = TemporaryString::from_to_params(text);

        if let Some(weak) = CACHE.get(&tmp) {
            if let Ok(strong) = weak.upgrade() {
                return Ok(Self::from_strong(strong));
            }
        }

        let mut strong_builder = StrongArrayBuilder::<u8>::default();
        strong_builder.set_additional_data(tmp.hash_value());
        let new_strong = strong_builder.build_from_copyable(text.as_bytes())?;
        let new_tmp = TemporaryString::from_to_params(&new_strong);

        CACHE.set(&new_tmp, new_strong.downgrade());

        Ok(Self::from_strong(new_strong))
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.array.as_slice()) }
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
    fn from_strong(strong: StrongArray<u8>) -> Self {
        Self {
            array: ManuallyDrop::new(strong),
        }
    }
}

impl Default for ImmutableString {
    fn default() -> Self {
        Self {
            array: ManuallyDrop::new(StrongArray::default()),
        }
    }
}

impl PartialEq for ImmutableString {
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array
    }
}

impl Eq for ImmutableString { }

impl Hash for ImmutableString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.array.additional_data().hash(state);
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
            .field("hash", &self.array.additional_data())
            .field("id", &self.id())
            .finish()
    }
}

impl Drop for ImmutableString {
    fn drop(&mut self) {
        let array = unsafe { ManuallyDrop::take(&mut self.array) };
        if let Some(unique) = array.release() {
            let key = TemporaryString::from_to_params(&unique);
            CACHE.remove(&key);
        }
    }
}
