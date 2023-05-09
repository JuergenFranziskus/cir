/// A calling convention a function adheres to.
/// Some of these are generic and may differ from target to target.
/// Others are concrete, and their exact details are always the same.
/// Both generic and concrete conventions may not be available on all targets.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CallingConvention {
    /// Use whatever calling convention is the default on the compilation target.
    /// Does not support var-args, since such support cannot be guaranteed on all targets.
    Default,
    /// Use the calling convention used by the C language on the compilation target.
    C,
    /// Use the calling convention defined by the X86_64 System-V ABI.
    SystemV,
}
impl CallingConvention {
    pub fn supports_varargs(self) -> bool {
        match self {
            Self::Default => false,
            Self::C => true,
            Self::SystemV => true,
        }
    }
}
impl Default for CallingConvention {
    fn default() -> Self {
        Self::Default
    }
}
