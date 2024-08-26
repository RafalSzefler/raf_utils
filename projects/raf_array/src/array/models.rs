use std::{
    alloc::Layout,
    marker::PhantomData,
    mem::forget,
    ptr::{self, null_mut}};


struct LayoutHelpers<T>(PhantomData<T>);

impl<T> LayoutHelpers<T> {
    const ALIGNMENT: usize = {
        let alignement = core::mem::align_of::<T>();
        let size = core::mem::size_of::<T>();
        assert!(alignement.is_power_of_two(), "ALIGNEMENT is not power of two.");
        assert!(size % alignement == 0, "size_of::<T>() not correctly aligned.");
        alignement
    };

    const fn layout(length: usize) -> Layout {
        unsafe {
            Layout::from_size_align_unchecked(
                length * core::mem::size_of::<T>(),
                Self::ALIGNMENT)
        }
    }
}

/// Represents internals of [`Array`]. This is a raw struct, doesn't contain
/// any logic inside, except it will get properly deallocated on drop.
#[derive(Debug)]
#[repr(C)]
pub struct ArrayPieces<T>
    where T: Sized
{
    pub ptr: *mut T,
    pub length: usize,
}

#[inline(always)]
fn empty_pieces<T>() -> ArrayPieces<T> {
    ArrayPieces { ptr: null_mut(), length: 0 }
}

impl<T> Drop for ArrayPieces<T>
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

        let layout = LayoutHelpers::<T>::layout(length);
        let raw_ptr = self.ptr.cast::<u8>();
        unsafe { std::alloc::dealloc(raw_ptr, layout) };
    }
}


/// Represents potential errors on array construction.
#[derive(Debug)]
#[repr(u8)]
pub enum ArrayNewError {
    AllocationError = 0,
    MaxLengthExceeded = 1,
}

/// Represents a dynamically created array with length known at runtime.
/// Generally a thin wrapper around slices. Similar to `Vec` but it cannot
/// change size.
#[repr(transparent)]
pub struct Array<T>
    where T: Sized
{
    pieces: ArrayPieces<T>
}

impl<T> Array<T>
    where T: Sized
{
    pub const fn max_len() -> usize { (i32::MAX - 1024) as usize }

    /// Creates a new instance of [`Array`]. It allocates the corresponding
    /// buffer on heap and fills it with values generated through `factory`.
    /// 
    /// # Errors
    /// * [`ArrayNewError::AllocationError`] when couldn't allocate internal
    ///   buffer, likely due to running out of memory.
    /// * [`ArrayNewError::MaxLengthExceeded`] when `length` is greater than
    ///   [`Array::max_len()`].
    pub fn from_factory<F>(length: usize, mut factory: F) -> Result<Self, ArrayNewError>
        where F: FnMut() -> T
    {        
        if length == 0 {
            return Ok(Self::default());
        }

        if length > Self::max_len() {
            return Err(ArrayNewError::MaxLengthExceeded);
        }

        let layout = LayoutHelpers::<T>::layout(length);
        let raw_ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        if raw_ptr.is_null() {
            return Err(ArrayNewError::AllocationError);
        }
        
        let buffer = raw_ptr.cast::<T>();
        let mut tmp_ptr = buffer;
        for _ in 0..length {
            unsafe {
                ptr::write(tmp_ptr, factory());
                tmp_ptr = tmp_ptr.add(1);
            }
        }
        let pieces = ArrayPieces { ptr: buffer, length: length };
        Ok(Self { pieces })
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            &*ptr::slice_from_raw_parts(self.pieces.ptr, self.pieces.length)
        }
    }

    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe {
            &mut *ptr::slice_from_raw_parts_mut(self.pieces.ptr, self.pieces.length)
        }
    }

    /// Returns the underlying [`ArrayPieces`] value.
    #[inline(always)]
    pub fn release(mut self) -> ArrayPieces<T> {
        let mut real_pieces = empty_pieces();
        unsafe { core::ptr::swap(&mut self.pieces, &mut real_pieces) };
        forget(self);
        real_pieces
    }
}

impl<T> Default for Array<T>
    where T: Sized
{
    #[inline]
    fn default() -> Self {
        Self { pieces: empty_pieces() }
    }
}

impl<T> core::fmt::Debug for Array<T>
    where T: Sized
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let pieces = &self.pieces;
        let numeric_value = core::ptr::addr_of!(pieces.ptr).cast::<usize>();
        f.debug_struct("Array")
            .field("address", &numeric_value)
            .field("length", &pieces.length)
            .finish()
    }
}
