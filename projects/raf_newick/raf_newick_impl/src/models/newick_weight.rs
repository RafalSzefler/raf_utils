#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct NewickWeight {
    integer_part: u32,
    fractional_part: u32,
}

impl NewickWeight {
    #[inline(always)]
    pub fn new(integer_part: u32, fractional_part: u32) -> Self {
        Self { integer_part, fractional_part }
    }

    #[inline(always)]
    pub fn integer_part(&self) -> u32 { self.integer_part }

    #[inline(always)]
    pub fn fractional_part(&self) -> u32 { self.fractional_part }
}
