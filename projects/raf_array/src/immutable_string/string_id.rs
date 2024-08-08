use crate::atomic_array::ArrayId;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct StringId {
    value: usize,
}

impl StringId {
    pub(super) fn from_array_id(id: ArrayId) -> Self {
        Self {
            value: id.value()
        }
    }

    pub fn value(&self) -> usize { self.value }
}
