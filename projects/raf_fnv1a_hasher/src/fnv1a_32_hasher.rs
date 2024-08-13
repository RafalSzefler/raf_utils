use core::hash::Hasher;

use crate::{calculations::update_fnv1a_32, init};

/// An implementation of 32-bit `FNV1a` algorithm.
pub struct FNV1a32Hasher {
    current_value: u32,
}

impl Hasher for FNV1a32Hasher {

    #[allow(clippy::cast_lossless)]
    fn finish(&self) -> u64 {
        self.current_value as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        update_fnv1a_32(
            &mut self.current_value,
            bytes);
    }
}

impl FNV1a32Hasher {
    /// Creates a new instance of [`FNV1a32Hasher`]. Unlike [`FNV1a32Hasher::default`]
    /// this function returns a random static state. Meaning the initial state
    /// will be randomized between different app lifetimes, but won't change
    /// during a single app lifetime.
    pub fn new() -> Self {
        let initial_value = init::get_initial_32_hash();
        Self { current_value: initial_value }
    }
}

impl Default for FNV1a32Hasher {
    /// Creates a new instance of [`FNV1a32Hasher`]. Unlike [`FNV1a32Hasher::new`]
    /// this function always returns the same static initial state, regardless
    /// of app it runs on.
    fn default() -> Self {
        Self { current_value: init::FNV1A_32_INITIAL }
    }
}
