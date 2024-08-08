use core::hash::{Hash, Hasher};

use super::{internal_array::InternalArray, NewStrongArrayError};

impl<T> PartialEq for InternalArray<T>
    where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T> Eq for InternalArray<T>
    where T: Eq
{ }

impl<T> Hash for InternalArray<T>
    where T: Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<T> Clone for InternalArray<T>
{
    fn clone(&self) -> Self {
        Self::raw_new(
            self.raw_ptr(),
            self.data_length() as u32)
    }
}

impl<T> InternalArray<T>
    where T: Clone
{
    pub fn clone_slice(arr: &[T]) -> Result<Self, NewStrongArrayError> {
        let length = arr.len();
        let mut result = Self::allocate_raw(length)?;
        let mut data_ptr = result.data_mut() as *mut T;
        for item in arr {
            unsafe {
                core::ptr::write(data_ptr, item.clone());
                data_ptr = data_ptr.add(1);
            }
        }
        Ok(result)
    }
}

impl<T> InternalArray<T>
    where T: Copy
{
    pub fn copy_slice(arr: &[T]) -> Result<Self, NewStrongArrayError> {
        let length = arr.len();
        let mut result = Self::allocate_raw(length)?;
        let data = result.as_slice_mut();
        data.copy_from_slice(arr);
        Ok(result)
    }
}
