use scapegoat::SgMap;
use crate::types::{KeyT, ValT};
use super::abstract_index::Index;

const CAPACITY: usize = 10;

pub struct ScapegoatIndex {
    index: SgMap<KeyT, ValT, CAPACITY>,
}

impl ScapegoatIndex {
    pub fn new() -> Self {
        ScapegoatIndex {
            index: SgMap::new(),
        }
    }

    pub fn remove(&mut self, key: &KeyT) -> Option<ValT> {
        self.index.remove(key)
    }

    pub fn contains_key(&self, key: &KeyT) -> bool {
        self.index.contains_key(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&KeyT, &ValT)> {
        self.index.iter()
    }

    pub fn clear(&mut self) {
        self.index.clear();
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }
}

impl Index for ScapegoatIndex {
    fn insert(&mut self, key: KeyT, val: ValT) {
        self.index.insert(key, val);
    }

    fn get(&self, key: &KeyT) -> Option<&ValT> {
        self.index.get(key)
    }
}