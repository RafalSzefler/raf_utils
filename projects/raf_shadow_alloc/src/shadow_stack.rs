#![allow(clippy::missing_panics_doc)]
use std::cell::UnsafeCell;

use region::{Allocation, Protection};

use crate::shadow_stack_size::get_shadow_stack_size;

struct ShadowStack {
    pub current_end: *mut u8,
    pub real_end: *mut u8,
    pub _region: Allocation,
}

thread_local! {
    static SHADOW_STACK: UnsafeCell<ShadowStack> = unsafe {
        let page_size = region::page::size();
        let size = get_shadow_stack_size() + page_size;
        let mut data = region::alloc(size, Protection::READ_WRITE).unwrap();

        #[allow(clippy::cast_sign_loss)]
        {
            let range = data.as_mut_ptr_range::<u8>();
            let slice = core::slice::from_raw_parts_mut(range.start, range.end.offset_from(range.start) as usize);
            slice.fill(0);
        }

        let raw = data.as_mut_ptr_range::<u8>();
        let real_end = raw.end.sub(page_size);
        region::protect(real_end, page_size, Protection::NONE).unwrap();
        let start = data.as_mut_ptr::<u8>();
        let stack = ShadowStack {
            current_end: start,
            real_end: real_end.sub(1),
            _region: data };
        UnsafeCell::new(stack)
    };
}

struct Guard<'a> {
    shadow_stack: &'a UnsafeCell<ShadowStack>,
    len: usize,
}

impl<'a> Drop for Guard<'a> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            let data = &mut *self.shadow_stack.get();
            data.current_end = data.current_end.sub(self.len);
        }
    }
}

macro_rules! shadow_alloc {
    ( $size: expr, $f1: expr, $f2: expr ) => {
        {
            SHADOW_STACK.try_with(|stack| {
                unsafe {
                    let size = { $size };
                    let f1 = { $f1 };
                    let mut f2 = { $f2 };
                    let data = &mut *stack.get();
                    let current = data.current_end;
                    let new_end = data.current_end.add(size);
                    assert!(new_end <= data.real_end, "Went over shadow stack limit.");
                    data.current_end = new_end;
                    let slice = core::slice::from_raw_parts_mut(current, size);
                    let _guard = Guard { shadow_stack: stack, len: size };
                    f1(slice);
                    f2(slice);
                }
            }).expect("Couldn't access Thread Local Storage.");
        }
    };
}

#[inline]
pub fn shadow_alloc<F>(size: usize, f: F)
    where F: FnMut(&mut [u8])
{
    #[inline(always)]
    fn dummy(_: &mut [u8]) {}

    shadow_alloc!(size, dummy, f);
}

#[inline]
pub fn shadow_alloc_zeroed<F>(size: usize, f: F)
    where F: FnMut(&mut [u8])
{
    #[inline(always)]
    fn zero(buf: &mut [u8]) { buf.fill(0); }

    shadow_alloc!(size, zero, f);
}


/// Returns available bytes in shadow stack.
#[allow(clippy::cast_sign_loss)]
#[inline]
pub fn get_available_shadow_stack_size() -> usize {
    SHADOW_STACK.with(|cell_stack| {
        unsafe {
            let stack = &*cell_stack.get();
            let diff = stack.real_end.offset_from(stack.current_end);
            assert!(diff >= 0, "Negative shadow stack size? Something went wrong.");
            diff as usize
        }
    })
}
