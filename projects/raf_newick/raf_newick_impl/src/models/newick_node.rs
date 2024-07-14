use super::{NewickName, NewickReticulation, NewickWeight, OptionalNewickReticulation, OptionalNewickWeight};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct NewickNodeId {
    value: i32
}

impl NewickNodeId {
    /// Builds new instance of [`NewickNodeId`].
    /// 
    /// # Safety
    /// Doesn't verify `value`. This `value` has additional semantics,
    /// that points to concrete element in node arrays. Use with caution.
    #[inline(always)]
    pub unsafe fn new_unchecked(value: i32) -> Self {
        Self { value }
    }

    #[inline(always)]
    pub fn value(&self) -> i32 { self.value }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct NewickNode {
    id: NewickNodeId,
    name: NewickName,
    weight: OptionalNewickWeight,
    reticulation: OptionalNewickReticulation,
}

impl NewickNode {
    /// Builds new instance of [`NewickNode`].
    /// 
    /// # Safety
    /// Doesn't verify validity of any of parameters.
    #[inline(always)]
    pub unsafe fn new_unchecked(
            id: NewickNodeId,
            name: NewickName,
            weight: OptionalNewickWeight,
            reticulation: OptionalNewickReticulation) -> Self
    {
        Self { id, name, weight, reticulation }
    }

    #[inline(always)]
    pub fn id(&self) -> NewickNodeId { self.id }

    #[inline(always)]
    pub fn name(&self) -> &NewickName { &self.name }

    #[inline(always)]
    pub fn weight(&self) -> Option<NewickWeight> {
        self.weight.as_option()
    }

    #[inline(always)]
    pub fn reticulation(&self) -> Option<&NewickReticulation> {
        self.reticulation.as_option()
    }
}
