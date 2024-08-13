use core::hash::{Hash, Hasher};

use super::{Array, ArrayNewError};

unsafe impl<T: Sync> Sync for Array<T> {}
unsafe impl<T: Send> Send for Array<T> {}

impl<T> Array<T>
    where T: Sized + Default
{
    /// Creates a new instance of [`Array`]. It allocates the corresponding
    /// buffer on heap and fills it with `T::default()`.
    /// 
    /// # Errors
    /// * [`ArrayNewError::AllocationError`] when couldn't allocate internal
    ///   buffer, likely due to running out of memory.
    /// * [`ArrayNewError::MaxLengthExceeded`] when `length` is greater than
    ///   [`Array::max_len()`].
    pub fn new_default(length: usize) -> Result<Self, ArrayNewError> {
        Self::from_factory(length, T::default)
    }
}


impl<T> PartialEq for Array<T>
    where T: Sized + PartialEq
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}


impl<T> Eq for Array<T>
    where T: Sized + Eq
{ }


impl<T> Hash for Array<T>
    where T: Sized + Hash
{
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<T> Clone for Array<T>
    where T: Sized + Clone
{
    fn clone(&self) -> Self {
        let mut idx = 0;
        let slice = self.as_slice();
        let factory = || {
            let result = slice[idx].clone();
            idx += 1;
            result
        };
        Array::from_factory(slice.len(), factory).unwrap()
    }
}
