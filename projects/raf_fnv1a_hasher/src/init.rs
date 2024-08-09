pub(crate) const FNV1A_32_INITIAL: u32 = 0x811c9dc5;

#[cfg(feature="getrandom")]
mod rand_impl {
    use std::sync::LazyLock;

    use crate::update_fnv1a_32;

    use super::FNV1A_32_INITIAL;

    fn create_random_u32_state() -> u32 {
        const RANDOM_STATE_SIZE: usize = 32;
        let mut array: [u8; RANDOM_STATE_SIZE] = [0; RANDOM_STATE_SIZE];
        getrandom::getrandom(&mut array).unwrap();
        
        let mut hash = FNV1A_32_INITIAL;
        update_fnv1a_32(&mut hash, &array);
        return hash;
    }

    static RANDOM_U32_STATE: LazyLock<u32>
        = LazyLock::new(create_random_u32_state);

    #[inline(always)]
    pub(super) fn init_32_hash() -> u32 { *RANDOM_U32_STATE }
}

#[cfg(not(feature="getrandom"))]
mod rand_impl {
    use super::FNV1A_32_INITIAL;

    #[inline(always)]
    pub(super) fn init_32_hash() -> u32 { FNV1A_32_INITIAL }
}

#[inline(always)]
pub(crate) fn get_initial_32_hash() -> u32 {
    rand_impl::init_32_hash()
}
