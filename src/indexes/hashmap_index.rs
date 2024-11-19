use crate::types::{KeyT, ValT};
use std::collections::HashMap;

use super::abstract_index::Index;

pub struct HashMapIndex {
    index: HashMap<KeyT, ValT>,
}

impl HashMapIndex {
    pub fn new() -> Self {
        HashMapIndex {
            index: HashMap::new(),
        }
    }
}

impl Index for HashMapIndex {
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