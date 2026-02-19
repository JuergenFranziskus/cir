#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Target {
    Hosted(Arch, Os),
}
impl Target {
    pub const LINUX_X64: Self = Self::Hosted(Arch::X86_64, Os::Linux);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Arch {
    X86_64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Os {
    Linux,
}
