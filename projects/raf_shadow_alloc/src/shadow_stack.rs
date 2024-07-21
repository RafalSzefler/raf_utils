
use std::cell::UnsafeCell;

use region::{Allocation, Protection};

use crate::shadow_stack_size::get_shadow_stack_size;

struct ShadowStack {
    pub current_end: *mut u8,
    pub _region: Allocation,
}

thread_local! {
    static SHADOW_STACK: UnsafeCell<ShadowStack> = unsafe {
        let page_size = region::page::size();
        let size = get_shadow_stack_size() + page_size;
        let mut data = region::alloc(size, Protection::READ_WRITE).unwrap();
        let raw = data.as_mut_ptr_range::<u8>();
        region::protect(raw.end.sub(page_size), page_size, Protection::NONE).unwrap();
        let start = data.as_mut_ptr::<u8>();
        let stack = ShadowStack { current_end: start, _region: data };
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

#[inline(always)]
fn shadow_alloc_with<F1, F2>(size: usize, mut f1: F1, mut f2: F2)
    where F1: FnMut(&mut [u8]),
        F2: FnMut(&mut [u8])
{
    SHADOW_STACK.try_with(|stack| {
        unsafe {
            let data = &mut *stack.get();
            let current = data.current_end;
            data.current_end = data.current_end.add(size);
            let slice = core::slice::from_raw_parts_mut(current, size);
            let _guard = Guard { shadow_stack: stack, len: size };
            f1(slice);
            f2(slice);
        }
    }).expect("Couldn't access Thread Local Storage.");
}

#[inline]
pub fn shadow_alloc<F>(size: usize, f: F)
    where F: FnMut(&mut [u8])
{
    #[inline(always)]
    fn dummy(_: &mut [u8]) {}

    shadow_alloc_with(size, dummy, f);
}

#[inline]
pub fn shadow_alloc_zeroed<F>(size: usize, f: F)
    where F: FnMut(&mut [u8])
{
    #[inline(always)]
    fn zero(buf: &mut [u8]) {
        buf.fill(0);
    }

    shadow_alloc_with(size, zero, f);
}
