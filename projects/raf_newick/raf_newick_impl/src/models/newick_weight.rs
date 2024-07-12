#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap)]

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct NewickWeight {
    integer_part: i32,
    fractional_part: i32,
}

impl NewickWeight {
    /// Contains max value for both integer and fractional part of weight.
    #[inline(always)]
    const fn max_value() -> u32 { (i32::MAX - 8) as u32 }

    /// Creates new [`NewickWeight`] instance.
    /// 
    /// # Panics
    /// If either `integer_part` or `fractional_part` exceeds
    /// [`NewickWeight::max_value()`].
    #[inline(always)]
    pub fn new(integer_part: u32, fractional_part: u32) -> Self {
        const MAX: u32 = NewickWeight::max_value();
        assert!(integer_part <= MAX,  "Max value for integer part is {MAX}.");
        assert!(fractional_part <= MAX, "Max value for fractional part is {MAX}.");

        unsafe {
            Self::new_unchecked(
                integer_part as i32,
                fractional_part as i32)
        }
    }

    #[inline(always)]
    pub fn integer_part(&self) -> u32 { self.integer_part as u32 }

    #[inline(always)]
    pub fn fractional_part(&self) -> u32 { self.fractional_part as u32 }

    #[inline(always)]
    unsafe fn new_unchecked(integer_part: i32, fractional_part: i32) -> Self {
        Self { integer_part, fractional_part }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct OptionalNewickWeight {
    weight: NewickWeight,
}

impl OptionalNewickWeight {
    #[inline(always)]
    pub fn some(weight: NewickWeight) -> Self {
        Self { weight }
    }

    #[inline(always)]
    pub fn none() -> Self {
        let weight = unsafe {
            NewickWeight::new_unchecked(-1, -1)
        };

        Self { weight }
    }

    #[inline(always)]
    pub fn weight(&self) -> Option<NewickWeight> {
        if self.weight.integer_part < 0 {
            None
        } else {
            Some(self.weight)
        }
    }
}
