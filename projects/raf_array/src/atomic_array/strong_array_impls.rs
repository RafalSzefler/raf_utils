#![allow(clippy::items_after_statements)]

use std::sync::LazyLock;

use const_format::formatcp;

use super::{internal_array::InternalArray, NewStrongArrayError, StrongArray};

impl<T> StrongArray<T>
    where T: Clone
{
    /// Creates a new instance of [`StrongArray`] by cloning slice.
    /// 
    /// # Notes
    /// This will allocate memory even when slice is empty. To get a shared
    /// empty array use [`StrongArray::default()`].
    /// 
    /// # Errors
    /// * [`NewStrongArrayError::MaxLengthExceeded`] if total byte length
    ///   exceeds [`StrongArray::max_byte_length()`].
    /// * [`NewStrongArrayError::AllocationError`] if couldn't allocate
    ///   underlying memory.
    /// * [`NewStrongArrayError::MisalignedResultError`] if allocator returned
    ///   a misaligned pointer.
    pub fn clone_slice(slice: &[T])
        -> Result<Self, NewStrongArrayError>
    {
        let internal = InternalArray::clone_slice(slice)?;
        Ok(Self::new_raw(internal))
    }
}

impl<T> StrongArray<T>
    where T: Copy
{
    /// Creates a new instance of [`StrongArray`] by copying slice.
    /// 
    /// # Notes
    /// This will allocate memory even when slice is empty. To get a shared
    /// empty array use [`StrongArray::default()`].
    /// 
    /// # Errors
    /// * [`NewStrongArrayError::MaxLengthExceeded`] if total byte length
    ///   exceeds [`StrongArray::max_byte_length()`].
    /// * [`NewStrongArrayError::AllocationError`] if couldn't allocate
    ///   underlying memory.
    /// * [`NewStrongArrayError::MisalignedResultError`] if allocator returned
    ///   a misaligned pointer.
    pub fn copy_slice(slice: &[T])
        -> Result<Self, NewStrongArrayError>
    {
        let internal = InternalArray::copy_slice(slice)?;
        Ok(Self::new_raw(internal))
    }
}

impl<T> StrongArray<T>
    where T: Default
{
    /// Creates a new instance of [`StrongArray`] by filling values with default
    /// value for T.
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
    pub fn new_default(length: usize)
        -> Result<Self, NewStrongArrayError>
    {
        let internal = InternalArray::generic_new(length, T::default)?;
        Ok(Self::new_raw(internal))
    }
}

#[repr(align(8192))]
struct MaxAlignmentStruct {
    _data: [u8; 8192],
}

#[derive(Clone, Copy)]
struct EmptyArrayCell {
    _data: [u8; size_of::<InternalArray<MaxAlignmentStruct>>()],
}

impl EmptyArrayCell {
    const _CHECK: () = {
        assert!(size_of::<MaxAlignmentStruct>() == align_of::<MaxAlignmentStruct>());
    };

    #[inline(always)]
    fn as_strong_array<T>(&self) -> StrongArray<T> {
        let internal = unsafe { core::mem::transmute::<EmptyArrayCell, InternalArray<T>>(*self) };
        let strong = StrongArray::new_raw(internal);
        let clone = strong.clone();
        core::mem::forget(strong);
        clone
    }
}

static LAZY_CELL: LazyLock<EmptyArrayCell>
    = LazyLock::new(|| {
        let internal_array = InternalArray::<MaxAlignmentStruct>::allocate_raw(0).unwrap();
        unsafe {
            core::mem::transmute::<InternalArray<MaxAlignmentStruct>, EmptyArrayCell>(internal_array)
        }
    });

impl<T> Default for StrongArray<T>
    where T: Sized
{
    /// Returns a new empty [`StrongArray`]. This array is shared, even between
    /// different `T`. In particular it will never get deallocated. It is
    /// constructed lazily. This is safe, since such array has no elements to
    /// work with, and its length cannot change. Strong/weak counters can still
    /// change, but they will never go to 0.
    fn default() -> Self {
        const MAX_ALIGNMENT: usize = align_of::<MaxAlignmentStruct>();
        let alignment: usize = align_of::<T>();
        const MSG: &str = formatcp!("Alignment of T can be at most {}.", MAX_ALIGNMENT);
        assert!(alignment <= MAX_ALIGNMENT, "{}", MSG);
        LAZY_CELL.as_strong_array()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let empty_i32 = StrongArray::<i32>::default();
        assert_eq!(empty_i32.strong_count(), 2);
        assert_eq!(empty_i32.as_slice(), []);
        let empty_i64 = StrongArray::<i64>::default();
        assert_eq!(empty_i32.strong_count(), 3);
        assert_eq!(empty_i64.strong_count(), 3);
        assert_eq!(empty_i64.as_slice(), []);
        let empty_string = StrongArray::<String>::default();
        assert_eq!(empty_i32.strong_count(), 4);
        assert_eq!(empty_i64.strong_count(), 4);
        assert_eq!(empty_string.strong_count(), 4);
        let empty_string_arr: &[String] = &[];
        assert_eq!(empty_string.as_slice(), empty_string_arr);
        assert_eq!(empty_i32.id(), empty_i64.id());
        assert_eq!(empty_string.id(), empty_i32.id());
        drop(empty_i32);
        assert_eq!(empty_i64.strong_count(), 3);
        assert_eq!(empty_string.strong_count(), 3);
        drop(empty_string);
        assert_eq!(empty_i64.strong_count(), 2);
    }
}
