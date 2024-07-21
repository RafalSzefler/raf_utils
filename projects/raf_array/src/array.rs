use std::{
    alloc::Layout,
    ptr::{self, null_mut}};

/// Represents a dynamically created array with length known at runtime.
/// Generally a thin wrapper around slices.
pub struct Array<T>
    where T: Sized
{
    ptr: *mut T,
    length: usize,
}

impl<T> Array<T>
    where T: Sized
{
    const ALIGNEMENT: usize = {
        let alignement = core::mem::align_of::<T>();
        let size = core::mem::size_of::<T>();
        assert!(alignement.is_power_of_two(), "ALIGNEMENT is not power of two.");
        assert!(size % alignement == 0, "size_of::<T>() not correctly aligned.");
        alignement
    };

    pub const fn max_len() -> usize { (i32::MAX - 1024) as usize }

    const fn layout(length: usize) -> Layout {
        unsafe {
            Layout::from_size_align_unchecked(
                length * core::mem::size_of::<T>(),
                Self::ALIGNEMENT)
        }
    }

    /// Creates a new instance of [`Array`]. It allocates the corresponding
    /// buffer on heap and fills it with values generated through `factory`.
    /// 
    /// # Panics
    /// Only when `length` is bigger than [`Array::max_len()`].
    pub fn new_with_fill<F>(length: usize, factory: F) -> Self
        where F: FnMut() -> T
    {
        assert!(length < Self::max_len(), "Length must be smaller than {}.", Self::max_len());

        if length == 0 {
            return Self::default()
        }

        let layout = Self::layout(length);
        let buffer = (unsafe { std::alloc::alloc_zeroed(layout) }).cast::<T>();
        let mut f = factory;
        let mut tmp_ptr = buffer;
        for _ in 0..length {
            unsafe {
                ptr::write(tmp_ptr, f());
                tmp_ptr = tmp_ptr.add(1);
            }
        }
        Self { ptr: buffer, length: length }
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            &*ptr::slice_from_raw_parts(self.ptr, self.length)
        }
    }

    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe {
            &mut *ptr::slice_from_raw_parts_mut(self.ptr, self.length)
        }
    }
}

impl<T> Drop for Array<T>
    where T: Sized
{
    fn drop(&mut self) {
        let length = self.length;
        if length == 0 {
            return;
        }
        
        unsafe {
            let mut real_length = length;
            let mut ptr = self.ptr;
            while real_length > 0 {
                ptr::drop_in_place(ptr);
                ptr = ptr.add(1);
                real_length -= 1;
            }
        }

        let layout = Self::layout(length);
        let raw_ptr = self.ptr.cast::<u8>();
        unsafe { std::alloc::dealloc(raw_ptr, layout) };
        self.ptr = null_mut();
        self.length = 0;
    }
}

impl<T> Default for Array<T>
    where T: Sized
{
    fn default() -> Self {
        Self { ptr: null_mut(), length: 0 }
    }
}

impl<T> core::fmt::Debug for Array<T>
    where T: Sized
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let numeric_value = core::ptr::addr_of!(self.ptr).cast::<usize>();
        f.debug_struct("Array")
            .field("address", &numeric_value)
            .field("length", &self.length)
            .finish()
    }
}
