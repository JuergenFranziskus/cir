

/// The layout of a type, consisting of a size and an alignment.
/// The alignment must always be a power of two.
/// The size, when rounded up to the alignment, does not overflow an i64.
/// The alignment does not overflow an i64.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TyLayout {
    size: u64,
    align: u64,
}
impl TyLayout {
    pub fn new(size: u64, align: u64) -> Self {
        assert!(align.is_power_of_two());
        let will = size.checked_next_multiple_of(align).unwrap();
        assert!(will < i64::MAX as u64);

        Self {
            size,
            align,
        }
    }
    pub fn new_signed(size: i64, align: i64) -> Self {
        let size = size.try_into().unwrap();
        let align = align.try_into().unwrap();
        Self::new(size, align)
    }

    pub fn size(self) -> u64 {
        self.size
    }
    pub fn align(self) -> u64 {
        self.align
    }
    pub fn bytes(self) -> (u64, u64) {
        (self.size, self.align)
    }
    pub fn bytes_signed(self) -> (i64, i64) {
        (self.size as i64, self.align as i64)
    }

    pub fn pad_to_align(self) -> Self {
        let size = self.size.checked_next_multiple_of(self.align).unwrap();
        Self::new(size, self.align)
    }
    pub fn extend(self, next: Self) -> (Self, u64) {
        let size = self.size.checked_next_multiple_of(next.align).unwrap();
        let offset = size;
        let align = self.align.max(next.align);
        let size = size + next.size;

        (Self::new(size, align), offset)
    }
    pub fn align_to(self, to: u64) -> Self {
        Self::new(self.size, to)
    }
}
