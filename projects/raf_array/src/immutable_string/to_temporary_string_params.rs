use core::hash::{Hash, Hasher};

use crate::atomic_array::{FinalStrongArray, StrongArray};

pub(super) struct ToTemporaryStringParamsResult<'a> {
    slice: &'a [u8],
    hash: u32,
}

impl<'a> ToTemporaryStringParamsResult<'a> {
    fn new(slice: &'a [u8], hash: u32) -> Self {
        Self { slice, hash }
    }

    #[inline(always)]
    pub fn slice(&self) -> &[u8] {
        self.slice
    }

    #[inline(always)]
    pub fn hash(&self) -> u32 {
        self.hash
    }
}

pub(super) trait ToTemporaryStringParams {
    fn to_params(&self) -> ToTemporaryStringParamsResult<'_>;
}

#[allow(clippy::cast_possible_truncation)]
#[inline(always)]
fn calculate_hash(slice: &[u8]) -> u32 {
    let mut hasher = raf_fnv1a_hasher::FNV1a32Hasher::new();
    slice.as_ref().hash(&mut hasher);
    let result = hasher.finish() as u32;
    if result == 0 { 1 } else { result }
}

impl ToTemporaryStringParams for str {
    #[inline(always)]
    fn to_params(&self) -> ToTemporaryStringParamsResult<'_> {
        let bytes = self.as_bytes();
        let hash = calculate_hash(bytes);
        ToTemporaryStringParamsResult::new(bytes, hash)
    }
}

impl ToTemporaryStringParams for FinalStrongArray<u8> {
    #[inline(always)]
    fn to_params(&self) -> ToTemporaryStringParamsResult<'_> {
        ToTemporaryStringParamsResult::new(
            self.as_slice(),
            self.additional_data(),
        )
    }
}

impl ToTemporaryStringParams for StrongArray<u8> {
    #[inline(always)]
    fn to_params(&self) -> ToTemporaryStringParamsResult<'_> {
        ToTemporaryStringParamsResult::new(
            self.as_slice(),
            self.additional_data(),
        )
    }
}
