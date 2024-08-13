use raf_array::immutable_string::ImmutableString;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct NewickReticulationKind {
    value: ImmutableString,
}

#[derive(Debug)]
pub enum NewReticulationKindError {
    TooLong,
    InternalAllocationError,
    NotAsciiAlphabetic,
}

impl NewickReticulationKind {
    pub const fn max_len() -> usize { 100 }

    /// Builds new instance of [`NewickReticulationKind`].
    /// 
    /// # Errors
    /// * [`NewReticulationKindError::TooLong`] if `text.len()` exceeds [`NewickReticulationKind::max_len()`]
    /// * [`NewReticulationKindError::NotAsciiAlphabetic`] if `text` contains non-ascii-alphabetic characters
    /// * [`NewReticulationKindError::InternalAllocationError`] if couldn't allocate buffer internally
    pub fn new(text: &str) -> Result<Self, NewReticulationKindError> {
        const MAX: usize = NewickReticulationKind::max_len();
        if text.len() > MAX {
            return Err(NewReticulationKindError::TooLong);
        }

        if !text.is_empty() && !text.chars().all(|chr| chr.is_ascii_alphabetic()) {
            return Err(NewReticulationKindError::NotAsciiAlphabetic);
        }
        
        let Ok(imm) = ImmutableString::new(text) else {
            return Err(NewReticulationKindError::InternalAllocationError);
        };

        Ok(Self { value: imm })
    }

    /// Builds new instance of [`NewickReticulationKind`].
    /// 
    /// # Safety
    /// The following have to be satisfied:
    /// * `text.len()` cannot exceed [`NewickReticulationKind::max_len()`]
    /// * `text` has to be ascii-alphabetic string
    /// 
    /// # Panics
    /// Only when can't allocate internal buffer.
    pub unsafe fn new_unchecked(text: &str) -> Self {
        let imm = ImmutableString::new(text)
            .expect("InternalAllocationError");
        Self { value: imm }
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str { self.value.as_str() }

    #[inline(always)]
    pub fn as_immutable_string(&self) -> &ImmutableString { &self.value }
}

impl Default for NewickReticulationKind {
    fn default() -> Self {
        Self { value: ImmutableString::default().clone() }
    }
}

impl core::fmt::Debug for NewickReticulationKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NewickReticulationKind")
            .field("value", &self.value.as_str())
            .finish()
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct NewickReticulation {
    id: u32,
    kind: NewickReticulationKind,
}

#[derive(Debug)]
pub enum NewReticulationError {
    IdZero,
    IdOutOfRange,
}

impl NewickReticulation {
    pub const fn max_id_value() -> u32 { (i32::MAX - 1024) as u32 }
    
    /// Builds new instance of [`NewickReticulation`].
    /// 
    /// # Errors
    /// * [`NewReticulationError::IdZero`] if `id == 0`
    /// * [`NewReticulationError::IdOutOfRange`] if `id` exceeds [`NewickReticulation::max_id_value()`]
    pub fn new(id: u32, kind: NewickReticulationKind)
        -> Result<Self, NewReticulationError>
    {
        if id == 0 {
            return Err(NewReticulationError::IdZero);
        }

        if id > Self::max_id_value() {
            return Err(NewReticulationError::IdOutOfRange);
        }

        let ret = unsafe { Self::new_unchecked(id, kind) };

        Ok(ret)
    }

    /// Builds new instance of [`NewickReticulation`].
    /// 
    /// # Safety
    /// The following have to be satisfied:
    /// * `id` is non-zero
    /// * `id` doesn't exceed [`NewickReticulation::max_id_value()`]
    pub unsafe fn new_unchecked(id: u32, kind: NewickReticulationKind)
        -> Self
    {
        Self {
            id: id,
            kind: kind
        }
    }

    #[inline(always)]
    pub fn id(&self) -> u32 { self.id }

    #[inline(always)]
    pub fn kind(&self) -> &NewickReticulationKind { &self.kind }
}

impl core::fmt::Debug for NewickReticulation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NewickReticulation")
            .field("id", &self.id)
            .field("kind", &self.kind)
            .finish()
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct OptionalNewickReticulation {
    value: NewickReticulation,
}

impl OptionalNewickReticulation {
    pub fn some(reticulation: NewickReticulation) -> Self {
        Self { value: reticulation }
    }

    pub fn none() -> Self { 
        unsafe {
            let empty_kind = NewickReticulationKind::new_unchecked("");
            let empty = NewickReticulation::new_unchecked(0, empty_kind);
            Self { value: empty }
        }
    }

    #[inline(always)]
    pub fn as_option(&self) -> Option<&NewickReticulation> {
        if self.value.id > 0 {
            Some(&self.value)
        } else {
            None
        }
    }
}

impl core::fmt::Debug for OptionalNewickReticulation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OptionalNewickReticulation")
            .field("option", &self.as_option())
            .finish()
    }
}
