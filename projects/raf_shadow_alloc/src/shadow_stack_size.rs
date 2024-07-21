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

static mut STACK_SIZE: StackSizeWrapper
    = StackSizeWrapper { cell: UnsafeCell::new(DEFAULT_STACK_SIZE) };

#[ctor]
fn initialize() {
    match env::var(STACK_SIZE_ENV_KEY) {
        Ok(val) => {
            match usize::from_str_radix(&val, 10) {
                Ok(value) => {
                    if value >= MIN_STACK_SIZE && value < MAX_STACK_SIZE {
                        unsafe {
                            *STACK_SIZE.cell.get_mut() = value;   
                        }
                    }
                },
                Err(_) => { },
            }
        },
        Err(_) => { }
    }
}

#[inline(always)]
pub fn get_shadow_stack_size() -> usize {
    unsafe { *STACK_SIZE.cell.get() }
}
