#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CallingConvention {
    Generic(GenericConvention),
    Concrete(ConcreteConvention),
}
impl CallingConvention {
    pub fn supports_varargs(self) -> bool {
        match self {
            Self::Generic(g) => g.supports_varargs(),
            Self::Concrete(c) => c.supports_varargs(),
        }
    }
}
impl Default for CallingConvention {
    fn default() -> Self {
        Self::Generic(GenericConvention::C)
    }
}

/// A calling convention that may differ in its details on different compilation targets.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GenericConvention {
    /// The calling convention used by the C language.
    /// Supports var-args
    C,
}
impl GenericConvention {
    pub fn supports_varargs(self) -> bool {
        match self {
            Self::C => true,
        }
    }
}

/// A calling convention whose exact details are known regardless of compilation target.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConcreteConvention {
    /// The calling convention specified by the x86_64 System-V ABI.
    /// Supports var-args.
    SystemV,
}
impl ConcreteConvention {
    pub fn supports_varargs(self) -> bool {
        match self {
            Self::SystemV => true,
        }
    }
}
