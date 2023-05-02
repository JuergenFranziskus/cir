#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CallingConvention {
    /// The calling convention used by the C language on the given target.
    /// The default when not explicitly set.
    CCC,
}
impl Default for CallingConvention {
    fn default() -> Self {
        Self::CCC
    }
}
