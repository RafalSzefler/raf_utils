use raf_immutable_string::ImmutableString;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct NewickName {
    imm: ImmutableString,
}

impl NewickName {
    /// Creates new instance of [`NewickName`]. Copies passed `text` to
    /// internal structures.
    /// 
    /// # Panics
    /// When can't allocate data internally for the copy.
    #[inline]
    pub fn new(text: &str) -> Self {
        let imm = ImmutableString::new(text)
            .expect("Couldn't create internal buffer.");
        Self { imm }
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str { self.imm.as_str() }
}

impl core::fmt::Debug for NewickName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NewickName")
            .field("value", &self.imm.as_str())
            .finish()
    }
}
