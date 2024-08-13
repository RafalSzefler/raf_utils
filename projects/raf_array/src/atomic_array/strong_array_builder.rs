use std::marker::PhantomData;

use super::{
    internal_array::InternalArray,
    NewStrongArrayError,
    StrongArray,
};

/// A builder for [`StrongArray`] instances.
pub struct StrongArrayBuilder<T>
    where T: Sized
{
    additional_data: u32,
    _phantom: PhantomData<T>,
}

impl<T> Default for StrongArrayBuilder<T>
    where T: Sized
{
    fn default() -> Self {
        Self { additional_data: 0, _phantom: PhantomData }
    }
}


impl<T> StrongArrayBuilder<T> {
    /// Sets additional data onto the newly constructed [`StrongArray`]. This
    /// can be any value and it is up to caller to interpret its meaning.
    pub fn set_additional_data(&mut self, additional_data: u32) {
        self.additional_data = additional_data;
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
    pub fn build_from_factory<TFactory>(
        self,
        length: usize,
        factory: TFactory)
    -> Result<StrongArray<T>, NewStrongArrayError>
        where TFactory: FnMut() -> T,
    {
        let internal = InternalArray::generic_new(
            length, self.additional_data, factory)?;
        Ok(StrongArray::new_raw(internal))
    }
}


impl<T> StrongArrayBuilder<T>
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
    pub fn build_default(self, length: usize)
        -> Result<StrongArray<T>, NewStrongArrayError>
    {
        let internal = InternalArray::generic_new(
            length, self.additional_data, T::default)?;
        Ok(StrongArray::new_raw(internal))
    }
}


impl<T> StrongArrayBuilder<T>
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
    pub fn build_from_copyable(self, slice: &[T])
        -> Result<StrongArray<T>, NewStrongArrayError>
    {
        let internal = InternalArray::copy_slice(slice, self.additional_data)?;
        Ok(StrongArray::new_raw(internal))
    }
}


impl<T> StrongArrayBuilder<T>
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
    pub fn build_from_clonable(self, slice: &[T])
        -> Result<StrongArray<T>, NewStrongArrayError>
    {
        let internal = InternalArray::clone_slice(slice, self.additional_data)?;
        Ok(StrongArray::new_raw(internal))
    }
}

