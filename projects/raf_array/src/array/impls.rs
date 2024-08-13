use core::hash::{Hash, Hasher};

use super::Array;

unsafe impl<T: Sync> Sync for Array<T> {}
unsafe impl<T: Send> Send for Array<T> {}

impl<T> Array<T>
    where T: Sized + Default
{
    /// Creates a new instance of [`Array`]. It allocates the corresponding
    /// buffer on heap and fills it with `T::default()`.
    /// 
    /// # Panics
    /// Only when `length` is bigger than [`Array::max_len()`].
    pub fn new(length: usize) -> Self {
        Self::new_with_fill(length, T::default)
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
        Array::new_with_fill(slice.len(), factory)
    }
}
