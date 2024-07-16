use raf_immutable_string::ImmutableString;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct NewickName {
    value: ImmutableString,
}

#[derive(Debug)]
pub enum NewNameError {
    TooLong,
    InternalAllocationError,
}

impl NewickName {
    pub const fn max_len() -> usize { (i32::MAX - 1024) as usize }

    /// Builds new instance of [`NewickName`].
    /// 
    /// # Errors
    /// * [`NewNameError::TooLong`] if `text.len()` exceeds [`NewickName::max_len()`]
    /// * [`NewNameError::InternalAllocationError`] if couldn't allocate buffer internally
    pub fn new(text: &str) -> Result<Self, NewNameError> {
        const MAX: usize = NewickName::max_len();
        if text.len() > MAX {
            return Err(NewNameError::TooLong);
        }
        
        let Ok(imm) = ImmutableString::new(text) else {
            return Err(NewNameError::InternalAllocationError);
        };

        Ok(Self { value: imm })
    }

    /// Builds new instance of [`NewickName`].
    /// 
    /// # Safety
    /// Length of `text` cannot exceed exceed [`NewickName::max_len()`] otherwise
    /// the behaviour is undefined.
    /// 
    /// # Panics
    /// Only when can't allocate internal buffer.
    #[inline(always)]
    pub unsafe fn new_unchecked(text: &str) -> NewickName {
        let imm = ImmutableString::new(text)
            .expect("InternalAllocationError");
        Self { value: imm }
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str { self.value.as_str() }

    #[inline(always)]
    pub fn as_immutable_string(&self) -> &ImmutableString { &self.value }
}

impl Default for NewickName {
    fn default() -> Self {
        Self { value: ImmutableString::empty().clone() }
    }
}

impl core::fmt::Debug for NewickName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NewickName")
            .field("value", &self.value.as_str())
            .finish()
    }
}
