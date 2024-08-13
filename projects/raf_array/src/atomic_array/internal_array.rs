#![allow(
    clippy::mut_from_ref,
    clippy::cast_ptr_alignment)]

use core::{
    alloc::Layout, marker::PhantomData, ptr::drop_in_place
};

use std::alloc::{alloc_zeroed, dealloc};

use super::{
    layout_holder::{
        AtomicType,
        InternalArrayLayout,
        LayoutHolder,
        MAX_PREFIX}, ArrayId, NewStrongArrayError};

#[inline(always)]
pub(super) const fn max_alloc_size() -> usize {
    (i32::MAX as usize) - MAX_PREFIX
}

#[allow(clippy::needless_pass_by_value)]
#[inline(always)]
fn as_usize<T>(raw: T) -> usize {
    unsafe { *core::ptr::from_ref(&raw).cast::<usize>() }
}


#[derive(Debug)]
pub(super) struct InternalArray<T>
    where T: Sized
{
    raw_ptr: *mut u8,
    length: u32,
    additional_data: u32,
    _phantom: PhantomData<T>,
}

// **********
// * PUBLIC *
// **********
impl<T> InternalArray<T>
{
    #[inline(always)]
    pub(super) fn raw_new(
        raw_ptr: *mut u8,
        length: u32,
        additional_data: u32
    ) -> Self {
        Self {
            raw_ptr: raw_ptr,
            length: length,
            additional_data: additional_data,
            _phantom: PhantomData
        }
    }

    #[inline(always)]
    pub fn strong_mut(&self) -> &mut AtomicType {
        let offset = Self::LAYOUT.strong_offset().value();
        unsafe {
            let ptr = self.raw_ptr.add(offset).cast::<AtomicType>();
            &mut *ptr
        }
    }

    #[inline(always)]
    pub fn weak_mut(&self) -> &mut AtomicType {
        let offset = Self::LAYOUT.weak_offset().value();
        unsafe {
            let ptr = self.raw_ptr.add(offset).cast::<AtomicType>();
            &mut *ptr
        }
    }

    #[inline(always)]
    pub fn data(&self) -> &T {
        let offset = Self::LAYOUT.data_offset().value();
        unsafe {
            let ptr = self.raw_ptr.add(offset).cast::<T>();
            &*ptr
        }
    }

    #[inline(always)]
    pub fn data_mut(&mut self) -> &mut T {
        let offset = Self::LAYOUT.data_offset().value();
        unsafe {
            let ptr = self.raw_ptr.add(offset).cast::<T>();
            &mut *ptr
        }
    }

    #[inline(always)]
    pub const fn data_length(&self) -> usize { self.length as usize }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        let len = self.data_length();
        let data = self.data();
        unsafe { core::slice::from_raw_parts(data, len) }
    }

    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        let len = self.data_length();
        let data = self.data_mut();
        unsafe { core::slice::from_raw_parts_mut(data, len) }
    }

    #[inline(always)]
    pub unsafe fn deallocate(&mut self) {
        for item in self.as_slice_mut() {
            drop_in_place(item);
        }
        let layout = Self::get_layout_for_length(self.length);
        dealloc(self.raw_ptr, layout);
    }

    #[inline(always)]
    pub fn id(&self) -> ArrayId {
        let numeric = as_usize(self.raw_ptr);
        ArrayId::new(numeric)
    }
    
    #[inline(always)]
    pub const fn additional_data(&self) -> u32 {
        self.additional_data
    }

    #[allow(clippy::cast_possible_truncation)]
    pub unsafe fn make_alias(&self) -> Self {
        Self::raw_new(
            self.raw_ptr,
            self.length,
            self.additional_data)
    }
}


// ****************
// * CONSTRUCTORS *
// ****************
impl<T> InternalArray<T> {
    #[allow(clippy::cast_possible_truncation)]
    #[inline(always)]
    pub fn allocate_raw(length: usize, additional_data: u32)
        -> Result<Self, NewStrongArrayError>
    {
        let layout = Self::get_layout_for_length(length as u32);
        if layout.size() > max_alloc_size() {
            return Err(NewStrongArrayError::MaxLengthExceeded);
        }

        let raw_ptr = unsafe { alloc_zeroed(layout) };
        if raw_ptr.is_null() {
            return Err(NewStrongArrayError::AllocationError);
        }

        let numeric_ptr = as_usize(raw_ptr);
        let alignment = Self::LAYOUT.total_alignment().value();
        if numeric_ptr % alignment != 0 {
            return Err(NewStrongArrayError::MisalignedResultError);
        }

        let result = Self::raw_new(raw_ptr, length as u32, additional_data);
        *result.strong_mut() = AtomicType::new(1);
        *result.weak_mut() = AtomicType::new(1);
        Ok(result)
    }

    #[inline(always)]
    pub fn generic_new<TFactory>(
            length: usize,
            additional_data: u32,
            mut factory: TFactory)
        -> Result<Self, NewStrongArrayError>
        where TFactory: FnMut() -> T
    {
        let mut result = Self::allocate_raw(length, additional_data)?;
        let mut data_ptr = result.data_mut() as *mut T;
        for _ in 0..length {
            unsafe {
                core::ptr::write(data_ptr, factory());
                data_ptr = data_ptr.add(1);
            }
        }
        Ok(result)
    }
}


// ***********
// * PRIVATE *
// ***********
impl<T> InternalArray<T> {
    const LAYOUT: InternalArrayLayout = LayoutHolder::<T>::LAYOUT;

    #[inline(always)]
    fn get_layout_for_length(length: u32) -> Layout {
        LayoutHolder::<T>::get_layout_for_length(length)
    }
}
