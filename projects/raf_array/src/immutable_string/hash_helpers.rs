use core::hash::{Hash, Hasher};

#[inline(always)]
pub(super) fn calculate_hash(slice: &[u8]) -> u64 {
    let mut hasher = raf_fnv1a_hasher::FNV1a32Hasher::new();
    slice.hash(&mut hasher);
    hasher.finish()
}
