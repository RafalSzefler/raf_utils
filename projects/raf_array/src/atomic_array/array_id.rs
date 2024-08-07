#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ArrayId {
    value: usize,
}

impl ArrayId {
    #[inline(always)]
    pub(super) fn new(value: usize) -> Self {
        Self { value }
    }

    pub fn value(&self) -> usize { self.value }
}
