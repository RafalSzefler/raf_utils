use core::{
    hash::{Hash, Hasher},
    mem::forget};

use super::{
    internal_array::{max_alloc_size, InternalArray},
    ArrayId,
    NewStrongArrayError,
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
    pub(super) fn new_raw(internal: InternalArray<T>) -> Self {
        Self { internal }
    }

    /// Maximal allowed size of array in bytes.
    pub const fn max_byte_length() -> usize {
        max_alloc_size()
    }

    /// Creates a new instance of [`StrongArray`].
    /// 
    /// # Notes
    /// This will allocate memory even when `length == 0`. To get a shared
    /// empty array use [`StrongArray::default()`].
    /// 
    /// # Errors
    /// * [`NewStrongArrayError::MaxLengthExceeded`] if total byte length
    ///   exceeds [`StrongArray::max_byte_length()`].
    /// * [`NewStrongArrayError::AllocationError`] if couldn't allocate
    ///   underlying memory.
    /// * [`NewStrongArrayError::MisalignedResultError`] if allocator returned
    ///   a misaligned pointer.
    pub fn new<TFactory>(
        length: usize,
        factory: TFactory)
    -> Result<Self, NewStrongArrayError>
        where TFactory: FnMut() -> T,
    {
        let internal = InternalArray::generic_new(length, factory)?;
        Ok(Self { internal })
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        self.internal.as_slice()
    }

    /// Returns new instance of [`WeakArray`].
    #[inline(always)]
    pub fn downgrade(&self) -> WeakArray<T> {
        let _ = self.internal.weak_mut().atomic_add(1);
        WeakArray::new_raw(self.internal.clone())
    }

    /// Releases current [`StrongArray`]. If it was the last [`StrongArray`]
    /// it returns the final [`UniqueStrongArray`] and [`None`] otherwise.
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
    fn release_mut(&mut self) -> Option<FinalStrongArray<T>> {
        let strong = self.internal.strong_mut();
        if strong.atomic_sub(1) == 1 {
            Some(FinalStrongArray::new_raw(self.internal.clone()))
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
        Self { internal: self.internal.clone() }
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
