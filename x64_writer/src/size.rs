#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Size {
    Byte,
    Word,
    Double,
    Quad,
}
impl Size {
    pub fn suffix(self) -> &'static str {
        match self {
            Size::Byte => "b",
            Size::Word => "w",
            Size::Double => "l",
            Size::Quad => "q",
        }
    }

    pub fn sandwich_affixes(self) -> (&'static str, &'static str) {
        use Size::*;
        match self {
            Byte => ("", "l"),
            Word => ("", "x"),
            Double => ("e", "x"),
            Quad => ("r", "x"),
        }
    }
    pub fn pointer_affixes(self) -> (&'static str, &'static str) {
        use Size::*;
        match self {
            Byte => ("", "l"),
            Word => ("", ""),
            Double => ("e", ""),
            Quad => ("r", ""),
        }
    }
    pub fn numbered_affixes(self) -> (&'static str, &'static str) {
        use Size::*;
        match self {
            Byte => ("", "b"),
            Word => ("", "w"),
            Double => ("", "d"),
            Quad => ("", ""),
        }
    }
    pub fn in_bytes(self) -> u8 {
        match self {
            Self::Byte => 1,
            Self::Word => 2,
            Self::Double => 4,
            Self::Quad => 8,
        }
    }
}
impl PartialOrd for Size {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        fn numeric(size: Size) -> u8 {
            match size {
                Size::Byte => 1,
                Size::Word => 2,
                Size::Double => 4,
                Size::Quad => 8,
            }
        }

        Some(numeric(*self).cmp(&numeric(*other)))
    }
}
impl Ord for Size {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
