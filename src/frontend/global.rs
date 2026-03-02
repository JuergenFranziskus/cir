use crate::frontend::Ty;


pub struct Global {
    pub id: GlobalID,
    pub name: Option<String>,
    pub ty: Ty,
    pub value: Option<GlobalValue>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlobalID(pub usize);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GlobalValue {
    Bool(bool),
    Int(i64),
    String(String),
}
impl From<&str> for GlobalValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
