use crate::merkle_tree::parallel_smt::ItemIndex;
use fnv::FnvHashMap;
use std::fmt::{Debug, Formatter};

pub trait SMTStorage<K, T>: Debug + Sync + Clone + Default {
    fn insert(&mut self, item_index: K, item: T);
    fn get(&self, index: &K) -> Option<&T>;
    fn remove(&mut self, index: &K) -> Option<T>;
    fn capacity(&self) -> usize;
}

pub struct DefaultMemorySMTStorage<T> {
    pub items: FnvHashMap<ItemIndex, T>,
}
unsafe impl<T> Send for DefaultMemorySMTStorage<T> {}

unsafe impl<T> Sync for DefaultMemorySMTStorage<T> {}

impl<T> Default for DefaultMemorySMTStorage<T> {
    fn default() -> Self {
        Self {
            items: Default::default(),
        }
    }
}
impl<T> Debug for DefaultMemorySMTStorage<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T: Clone> Clone for DefaultMemorySMTStorage<T> {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
        }
    }
}

impl<T: Clone> SMTStorage<ItemIndex, T> for DefaultMemorySMTStorage<T> {
    fn insert(&mut self, item_index: ItemIndex, item: T) {
        self.items.insert(item_index, item);
    }

    fn get(&self, index: &ItemIndex) -> Option<&T> {
        self.items.get(index)
    }

    fn remove(&mut self, index: &ItemIndex) -> Option<T> {
        self.items.remove(index)
    }

    fn capacity(&self) -> usize {
        self.items.capacity()
    }
}
