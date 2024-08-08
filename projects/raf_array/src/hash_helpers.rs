use core::hash::{Hash, Hasher};

#[inline(always)]
pub(crate) fn calculate_hash(slice: &[u8]) -> u32
{
    let mut hasher = raf_fnv1a_hasher::FNV1a32Hasher::new();
    slice.as_ref().hash(&mut hasher);
    let result = hasher.finish() as u32;
    if result == 0 { 1 } else { result }
}
