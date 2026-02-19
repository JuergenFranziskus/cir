use super::{function::FunID, types::Ty};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub id: VarID,
    pub fun: FunID,
    pub ty: Ty,
}
impl Variable {
    pub(crate) fn new(id: VarID, fun: FunID, ty: Ty) -> Self {
        Self { id, fun, ty }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VarID(pub(crate) usize);
