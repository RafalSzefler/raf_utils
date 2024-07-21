use std::{cell::UnsafeCell, env};

use ctor::ctor;

const STACK_SIZE_ENV_KEY: &str = "RAF_BUFFER_POOL_SIZE";
const MIN_STACK_SIZE: usize = 64;
const DEFAULT_STACK_SIZE: usize = 1 << 20;
const MAX_STACK_SIZE: usize = 1 << 30;

struct StackSizeWrapper {
    pub cell: UnsafeCell<usize>,
}

unsafe impl Sync for StackSizeWrapper {}
unsafe impl Send for StackSizeWrapper {}

static STACK_SIZE: StackSizeWrapper
    = StackSizeWrapper { cell: UnsafeCell::new(DEFAULT_STACK_SIZE) };

#[ctor]
fn initialize() {
    if let Ok(env_val) = env::var(STACK_SIZE_ENV_KEY) {
        if let Ok(value) = env_val.parse::<usize>() {
            if (MIN_STACK_SIZE..MAX_STACK_SIZE).contains(&value) {
                unsafe { core::ptr::write(STACK_SIZE.cell.get(), value) };
            }
        }
    }
}

/// Returns the total size of the shadow stack.
#[inline(always)]
pub fn get_shadow_stack_size() -> usize {
    unsafe { *STACK_SIZE.cell.get() }
}
