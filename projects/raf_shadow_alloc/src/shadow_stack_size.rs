use std::{env, sync::LazyLock};

const STACK_SIZE_ENV_KEY: &str = "RAF_BUFFER_POOL_SIZE";
const MIN_STACK_SIZE: usize = 64;
const DEFAULT_STACK_SIZE: usize = 1 << 20;
const MAX_STACK_SIZE: usize = 1 << 30;

static STACK_SIZE: LazyLock<usize>
    = LazyLock::new(|| {
        if let Ok(env_val) = env::var(STACK_SIZE_ENV_KEY) {
            if let Ok(value) = env_val.parse::<usize>() {
                if (MIN_STACK_SIZE..MAX_STACK_SIZE).contains(&value) {
                    return value;
                }
            }
        }
        DEFAULT_STACK_SIZE    
    });

/// Returns the total size of the shadow stack.
#[inline(always)]
pub fn get_shadow_stack_size() -> usize { *STACK_SIZE }
