#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct NewickNode {
    id: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum NewickNodeNewError {
    NegativeId,
    MaxIdValueExceeded,
}

impl NewickNode {
    /// Returns max numerical id value for a [`NewickNode`].
    #[inline(always)]
    pub const fn max_id_value() -> i32 { 1 << 30 }

    /// Creates new [`NewickNode`] out of `id: i32`.
    /// 
    /// # Errors
    /// * [`NewickNodeNewError::NegativeId`] when passed `id` is negative
    /// * [`NewickNodeNewError::MaxIdValueExceeded`] when passed `id` exceeds
    /// [`NewickNode::max_id_value()`].
    #[inline]
    pub fn new(id: i32) -> Result<Self, NewickNodeNewError> {
        if id < 0 {
            return Err(NewickNodeNewError::NegativeId);
        }
        if id > Self::max_id_value() {
            return Err(NewickNodeNewError::MaxIdValueExceeded);
        }
        Ok( Self { id } )
    }

    /// Creates new [`NewickNode`] out of `id: i32`.
    /// 
    /// # Safety
    /// It is up to caller to ensure that `id` is non-negative and doesn't
    /// exceed [`NewickNode::max_id_value()`].
    #[inline(always)]
    pub unsafe fn new_unchecked(id: i32) -> Self {
        Self { id }
    }

    #[inline(always)]
    pub fn id(&self) -> i32 { self.id }
}
