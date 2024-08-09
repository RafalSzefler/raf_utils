use core::{
    hash::{Hash, Hasher},
    mem::forget};

use super::{
    internal_array::{max_alloc_size, InternalArray},
    ArrayId,
    FinalStrongArray,
    WeakArray};

#[allow(unused_imports)]
use crate::array::Array;

/// Similar to [`Array`], except backed by atomic reference counters.
pub struct StrongArray<T>
    where T: Sized
{
    internal: InternalArray<T>
}


impl<T> StrongArray<T> {
    /// Maximal allowed size of array in bytes.
    pub const fn max_byte_length() -> usize {
        max_alloc_size()
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        self.internal.as_slice()
    }

    /// Returns new instance of [`WeakArray`].
    #[inline(always)]
    pub fn downgrade(&self) -> WeakArray<T> {
        let _ = self.internal.weak_mut().atomic_add(1);
        WeakArray::new_raw(unsafe { self.internal.make_alias() })
    }

    /// Releases current [`StrongArray`]. If it was the last [`StrongArray`]
    /// it returns the final [`FinalStrongArray`] and [`None`] otherwise.
    #[must_use]
    #[inline(always)]
    pub fn release(mut self) -> Option<FinalStrongArray<T>> {
        let result = self.release_mut();
        forget(self);
        result
    }

    /// Returns a unique identifier for this array. It is shared between
    /// strong/weak references.
    #[inline(always)]
    pub fn id(&self) -> ArrayId { self.internal.id() }

    /// Returns the number of alive strong references.
    #[inline(always)]
    pub fn strong_count(&self) -> u32 {
        self.internal.strong_mut().atomic_load()
    }

    /// Returns the number of alive weak references.
    #[inline(always)]
    pub fn weak_count(&self) -> u32 {
        self.internal.weak_mut().atomic_load()
    }

    #[inline(always)]
    pub fn additional_data(&self) -> u32 {
        self.internal.additional_data()
    }

    #[inline(always)]
    pub(super) fn new_raw(internal: InternalArray<T>) -> Self {
        Self { internal }
    }

    #[inline(always)]
    fn release_mut(&mut self) -> Option<FinalStrongArray<T>> {
        let strong = self.internal.strong_mut();
        if strong.atomic_sub(1) == 1 {
            Some(FinalStrongArray::new_raw(unsafe { self.internal.make_alias() }))
        } else {
            None
        }
    }
}

impl<T> Drop for StrongArray<T> {
    fn drop(&mut self) {
        let _ = self.release_mut();
    }
}

impl<T> Clone for StrongArray<T> {
    /// Clones current [`StrongArray`] by bumping internal strong ref counter.
    fn clone(&self) -> Self {
        let _ = self.internal.strong_mut().atomic_add(1);
        Self {
            internal: unsafe { self.internal.make_alias() }
        }
    }
}

impl<T> PartialEq for StrongArray<T>
    where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.internal == other.internal
    }
}

impl<T> Eq for StrongArray<T>
    where T: Eq
{ }

impl<T> Hash for StrongArray<T>
    where T: Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.internal.hash(state);
    }
}

unsafe impl<T> Sync for StrongArray<T> where T: Sync + Send { }
unsafe impl<T> Send for StrongArray<T> where T: Sync + Send { }

impl<T> core::fmt::Debug for StrongArray<T>
    where T: core::fmt::Debug
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StrongArray")
            .field("internal", &self.internal).finish()
    }
}
