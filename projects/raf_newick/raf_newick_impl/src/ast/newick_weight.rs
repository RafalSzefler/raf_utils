#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap)]

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct NewickWeight {
    integral_part: i32,
    fractional_part: i32,
}

#[derive(Debug)]
pub enum NewWeightError {
    IntegralPartOutOfRange,
    FractionalPartOutOfRange,
}

impl NewickWeight {
    pub const fn max_value() -> u32 { (i32::MAX - 1) as u32 }

    /// Builds new instance of [`NewickWeight`].
    /// 
    /// # Errors
    /// * [`NewWeightError::IntegralPartOutOfRange`] if `integral_part` exceeds [`NewickWeight::max_value()`]
    /// * [`NewWeightError::FractionalPartOutOfRange`] if `fractional_part` exceeds [`NewickWeight::max_value()`]
    pub fn new(integral_part: u32, fractional_part: u32)
        -> Result<Self, NewWeightError>
    {
        if integral_part > NewickWeight::max_value() {
            return Err(NewWeightError::IntegralPartOutOfRange);
        }

        if fractional_part > NewickWeight::max_value() {
            return Err(NewWeightError::FractionalPartOutOfRange);
        }

        let result = unsafe {
            Self::new_unchecked(integral_part, fractional_part)
        };

        Ok(result)
    }

    /// Builds new instance of [`NewickWeight`].
    /// 
    /// # Safety
    /// Both `integral_part` and `fractional_part` cannot exceed [`NewickWeight::max_value()`]
    #[inline(always)]
    pub unsafe fn new_unchecked(integral_part: u32, fractional_part: u32)
        -> Self
    {
        Self {
            integral_part: integral_part as i32,
            fractional_part: fractional_part as i32 }
    }

    fn empty() -> Self {
        Self { integral_part: -1, fractional_part: -1 }
    }

    #[inline(always)]
    pub fn integral_part(&self) -> u32 { self.integral_part as u32 }

    #[inline(always)]
    pub fn fractional_part(&self) -> u32 { self.fractional_part as u32 }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct OptionalNewickWeight {
    value: NewickWeight,
}

impl OptionalNewickWeight {
    #[inline(always)]
    pub fn some(value: NewickWeight) -> Self {
        Self { value: value }
    }

    #[inline(always)]
    pub fn none() -> Self {
        Self { value: NewickWeight::empty() }
    }

    #[inline(always)]
    pub fn as_option(&self) -> Option<NewickWeight> {
        if self.value.integral_part == -1 {
            None
        } else {
            Some(self.value)
        }
    }
}

impl core::fmt::Debug for OptionalNewickWeight {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OptionalNewickWeight")
            .field("option", &self.as_option())
            .finish()
    }
}
