use super::args::Memory;
use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Label<'a> {
    pub label: &'a str,
}
impl<'a> Label<'a> {
    pub fn new(label: &'a str) -> Self {
        Self { label }
    }

    pub fn rip(&self) -> Memory<'a> {
        Memory::rip().label(self.label)
    }
}
impl<'a> From<&'a str> for Label<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value)
    }
}
impl<'a> From<&'a String> for Label<'a> {
    fn from(value: &'a String) -> Self {
        Self { label: value }
    }
}
impl Display for Label<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}
