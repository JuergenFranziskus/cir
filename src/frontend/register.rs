use super::{function::FunID, types::Ty};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Register {
    pub id: RegID,
    pub fun: FunID,
    pub ty: Ty,
}
impl Register {
    pub(crate) fn new(id: RegID, func: FunID, ty: Ty) -> Self {
        Self { id, fun: func, ty }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RegID(pub(crate) usize);
