use crate::types::{KeyT, ValT};
use splay::SplayMap;

use super::abstract_index::Index;

pub struct SplayTreeIndex {
    index: SplayMap<KeyT, ValT>,
}

impl SplayTreeIndex {
    pub fn new() -> Self {
        SplayTreeIndex {
            index: SplayMap::new(),
        }
    }
}

impl Index for SplayTreeIndex {
    fn insert(&mut self, key: KeyT, val: ValT) {
        self.index.insert(key, val);
    }

    fn get(&self, key: &KeyT) -> Option<&ValT> {
        self.index.get(key)
    }

    fn clear(&mut self) -> () {
        self.index.clear();
    }
}
