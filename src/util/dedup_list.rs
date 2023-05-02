use std::ops::{Index, IndexMut};

pub struct DedupList<T> {
    items: Vec<T>,
}
impl<T> DedupList<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn clear(&mut self) {
        self.items.clear();
    }
}
impl<T: DedupItem> DedupList<T> {
    pub fn insert(&mut self, make: impl FnOnce(usize) -> T) -> usize {
        let pot_id = self.items.len();
        let pot_item = make(pot_id);

        // TODO: This is horribly slow probably, fix this
        if let Some((i, _)) = self
            .items
            .iter()
            .enumerate()
            .find(|(_i, item)| item.is_equivalent_to(&pot_item))
        {
            i
        } else {
            self.items.push(pot_item);
            pot_id
        }
    }
}
impl<T> Index<usize> for DedupList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}
impl<T> IndexMut<usize> for DedupList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}

pub trait DedupItem {
    fn hash_sans_id(&self) -> u64;
    fn is_equivalent_to(&self, other: &Self) -> bool;
}
