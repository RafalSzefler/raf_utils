use crate::atomic_array::ArrayId;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(transparent)]
pub struct StringId {
    value: usize,
}

impl StringId {
    #[inline(always)]
    pub(super) const fn from_array_id(id: ArrayId) -> Self {
        Self {
            value: id.value()
        }
    }

    #[inline(always)]
    pub const fn value(&self) -> usize { self.value }
}
