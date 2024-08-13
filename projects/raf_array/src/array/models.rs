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


/// Represents a dynamically created array with length known at runtime.
/// Generally a thin wrapper around slices. Similar to `Vec` but it cannot
/// change size.
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
    /// # Panics
    /// Only when `length` is bigger than [`Array::max_len()`].
    pub fn new_with_fill<F>(length: usize, factory: F) -> Self
        where F: FnMut() -> T
    {
        assert!(length < Self::max_len(), "Length must be smaller than {}.", Self::max_len());

        if length == 0 {
            return Self::default()
        }

        let layout = LayoutHelpers::<T>::layout(length);
        let buffer = (unsafe { std::alloc::alloc_zeroed(layout) }).cast::<T>();
        let mut f = factory;
        let mut tmp_ptr = buffer;
        for _ in 0..length {
            unsafe {
                ptr::write(tmp_ptr, f());
                tmp_ptr = tmp_ptr.add(1);
            }
        }
        let pieces = ArrayPieces { ptr: buffer, length: length };
        Self { pieces }
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
