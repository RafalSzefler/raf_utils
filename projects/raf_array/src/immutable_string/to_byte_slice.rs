use crate::atomic_array::{FinalStrongArray, StrongArray};

pub(super) trait ToByteSlice {
    fn to_slice(&self) -> &[u8];
}

impl ToByteSlice for str {
    #[inline(always)]
    fn to_slice(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl ToByteSlice for StrongArray<u8> {
    #[inline(always)]
    fn to_slice(&self) -> &[u8] {
        self.as_slice()
    }
}

impl ToByteSlice for FinalStrongArray<u8> {
    fn to_slice(&self) -> &[u8] {
        self.as_slice()
    }
}
