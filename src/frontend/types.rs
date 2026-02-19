use std::ops::Index;

use crate::target::Target;

use super::CallConvention;

pub struct Types {
    func_types: Vec<FunTy>,
    array_types: Vec<ArrayTy>,
    struct_types: Vec<StructTy>,
}
impl Types {
    pub fn new() -> Self {
        Self {
            func_types: Vec::new(),
            array_types: Vec::new(),
            struct_types: Vec::new(),
        }
    }

    pub fn add_func_type(
        &mut self,
        call_convention: CallConvention,
        ret: Ty,
        params: impl Into<Vec<Ty>>,
    ) -> FunTyID {
        let func_type = FunTy {
            call_convention,
            ret,
            params: params.into(),
        };
        let types = &mut self.func_types;

        for (i, ty) in types.iter().enumerate() {
            if ty == &func_type {
                return FunTyID(i);
            }
        }

        let i = types.len();
        types.push(func_type);
        FunTyID(i)
    }
    pub fn add_array_type(&mut self, size: u64, element: Ty) -> ArrayTyID {
        let types = &mut self.array_types;
        let array_ty = ArrayTy { size, element };

        for (i, ty) in types.iter().enumerate() {
            if ty == &array_ty {
                return ArrayTyID(i);
            }
        }

        let i = types.len();
        types.push(array_ty);
        ArrayTyID(i)
    }
    pub fn add_struct_type(&mut self) -> StructTyID {
        let id = StructTyID(self.struct_types.len());
        self.struct_types.push(StructTy {
            members: Vec::new(),
        });
        id
    }

    pub fn add_struct_member(&mut self, strct: StructTyID, member: Ty) {
        self.struct_types[strct.0].members.push(member);
    }

    pub fn layout(&self, ty: Ty, target: Target) -> (u64, u64) {
        assert_eq!(target, Target::LINUX_X64);
        match ty {
            Ty::Void => (0, 1),
            Ty::Bool => (1, 1),
            Ty::Ptr => (8, 8),
            Ty::Int(IntTy::I8) => (1, 1),
            Ty::Int(IntTy::I16) => (2, 2),
            Ty::Int(IntTy::I32) => (4, 4),
            Ty::Int(IntTy::I64) => (8, 8),
            Ty::Array(id) => {
                let (size, align) = self.layout(self[id].element, target);
                let size = size * self[id].size;
                (size, align)
            }
            Ty::Struct(id) => {
                let mut size = 0;
                let mut align = 1;

                for &member in &self[id].members {
                    let (msize, malign) = self.layout(member, target);
                    while size % malign != 0 {
                        size += 1;
                    }
                    size += msize;
                    align = align.max(malign);
                }

                while size % align != 0 {
                    size += 1;
                }

                (size, align)
            }
        }
    }
}
impl Index<FunTyID> for Types {
    type Output = FunTy;
    fn index(&self, index: FunTyID) -> &Self::Output {
        &self.func_types[index.0]
    }
}
impl Index<ArrayTyID> for Types {
    type Output = ArrayTy;
    fn index(&self, index: ArrayTyID) -> &Self::Output {
        &self.array_types[index.0]
    }
}
impl Index<StructTyID> for Types {
    type Output = StructTy;
    fn index(&self, index: StructTyID) -> &Self::Output {
        &self.struct_types[index.0]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ty {
    Void,
    Bool,
    Ptr,
    Int(IntTy),
    Array(ArrayTyID),
    Struct(StructTyID),
}
impl From<IntTy> for Ty {
    fn from(value: IntTy) -> Self {
        Self::Int(value)
    }
}
impl From<ArrayTyID> for Ty {
    fn from(value: ArrayTyID) -> Self {
        Self::Array(value)
    }
}
impl From<StructTyID> for Ty {
    fn from(value: StructTyID) -> Self {
        Self::Struct(value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IntTy {
    I8,
    I16,
    I32,
    I64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunTy {
    pub call_convention: CallConvention,
    pub ret: Ty,
    pub params: Vec<Ty>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunTyID(usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArrayTy {
    pub size: u64,
    pub element: Ty,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArrayTyID(usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructTy {
    pub members: Vec<Ty>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructTyID(usize);
