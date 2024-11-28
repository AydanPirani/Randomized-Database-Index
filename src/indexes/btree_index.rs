use crate::types::{KeyT, ValT};
use std::collections::BTreeMap;

use super::abstract_index::Index;

pub struct BTreeIndex {
    index: BTreeMap<KeyT, ValT>,
}

impl BTreeIndex {
    pub fn new() -> Self {
        BTreeIndex {
            index: BTreeMap::new(),
        }
    }
}

impl Index for BTreeIndex {
    fn insert(&mut self, key: KeyT, val: ValT) {
        self.index.insert(key, val);
    }

    fn get(&mut self, key: &KeyT) -> Option<&ValT> {
        return self.index.get(key);
    }

    fn clear(&mut self) -> () {
        self.index.clear();
    }
}