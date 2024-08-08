use crate::atomic_array::WeakArray;

#[derive(Clone)]
pub(super) struct WeakString {
    array: WeakArray<u8>,
}

impl WeakString {
    #[inline(always)]
    pub fn from_weak_array(array: WeakArray<u8>) -> Self {
        Self {
            array: array,
        }
    }

    #[inline(always)]
    pub fn array(&self) -> &WeakArray<u8> {
        &self.array
    }
}
