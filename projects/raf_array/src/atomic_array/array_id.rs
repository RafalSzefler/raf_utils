/// Represents a unique identifier of a (shared) array. Two arrays with the
/// same id are necessarily equal, although the same is not true in the
/// opposite.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(transparent)]
pub struct ArrayId {
    value: usize,
}

impl ArrayId {
    #[inline(always)]
    pub(super) fn new(value: usize) -> Self {
        Self { value }
    }

    #[inline(always)]
    pub const fn value(&self) -> usize { self.value }
}
