#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Target {
    RealSystem(Architecture, System),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Architecture {
    X86_64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum System {
    BareMetal,
    Linux,
}
