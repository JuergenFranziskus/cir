use crate::{util::dedup_list::DedupItem, Type};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub struct StructType {
    id: StructTypeID,
    members: Vec<Type>,
}
impl StructType {
    pub(super) fn new(id: StructTypeID, members: Vec<Type>) -> Self {
        Self { id, members }
    }

    pub fn id(&self) -> StructTypeID {
        self.id
    }
    pub fn members(&self) -> &[Type] {
        &self.members
    }
}
impl DedupItem for StructType {
    fn hash_sans_id(&self) -> u64 {
        let mut state = DefaultHasher::new();
        self.members.hash(&mut state);
        state.finish()
    }

    fn is_equivalent_to(&self, other: &Self) -> bool {
        self.members == other.members
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructTypeID(pub usize);
