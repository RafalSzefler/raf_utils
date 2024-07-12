use raf_immutable_string::ImmutableString;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct NewickReticulationType {
    imm: ImmutableString,
}

impl NewickReticulationType {
    const fn max_len() -> usize { 100 }

    /// Creates new instance of [`NewickReticulationType`]. Copies passed `text` to
    /// internal structures.
    /// 
    /// # Panics
    /// * when text is empty
    /// * when text length exceeds [`NewickReticulationType::max_len()`]
    /// * when text not `char::is_ascii_alphabetic`
    /// * when can't allocate data internally for the copy.
    #[inline]
    pub fn new(text: &str) -> Self {
        const MAX: usize = NewickReticulationType::max_len();
        assert!(!text.is_empty(), "Text empty");
        assert!(text.len() <= MAX, "Text length exceeds maximum of {MAX}");
        assert!(text.chars().all(|x| char::is_ascii_alphabetic(&x)), "Text not is_ascii_alphabetic.");
        let imm = ImmutableString::new(text)
            .expect("Couldn't create internal buffer.");
        Self { imm }
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str { self.imm.as_str() }
}

impl core::fmt::Debug for NewickReticulationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NewickReticulationType")
            .field("value", &self.imm.as_str())
            .finish()
    }
}
