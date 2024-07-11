use super::{NewickNode, NewickWeight};


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct NewickArrow {
    source: NewickNode,
    target: NewickNode,
    weight: NewickWeight,
}

impl NewickArrow {
    /// Creates new [`NewickArrow`].
    #[inline(always)]
    pub fn new(source: NewickNode, target: NewickNode, weight: NewickWeight) -> Self {
        Self { source, target, weight }
    }

    #[inline(always)]
    pub fn source(&self) -> NewickNode { self.source }

    #[inline(always)]
    pub fn target(&self) -> NewickNode { self.target }

    #[inline(always)]
    pub fn weight(&self) -> NewickWeight { self.weight }
}
