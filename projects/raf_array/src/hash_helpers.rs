use core::hash::{Hash, Hasher};

#[inline(always)]
pub(crate) fn calculate_hash<T>(slice: &[T]) -> u32
    where T: Hash
{
    let mut hasher = raf_fnv1a_hasher::FNV1a32Hasher::new();
    slice.hash(&mut hasher);
    let result = hasher.finish() as u32;
    if result == 0 { 1 } else { result }
}
