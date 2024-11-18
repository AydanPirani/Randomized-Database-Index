use crate::types::{KeyT, ValT};
use skiplist::SkipMap;

use super::abstract_index::Index;

pub struct SkipListIndex {
    index: SkipMap<KeyT, ValT>,
}

impl SkipListIndex {
    pub fn new() -> Self {
        SkipListIndex {
            index: SkipMap::new(),
        }
    }
}

impl Index for SkipListIndex {
    fn insert(&mut self, key: KeyT, val: ValT) {
        self.index.insert(key, val);
    }

    fn get(&self, key: &KeyT) -> Option<&ValT> {
        return self.index.get(key);
    }

    fn clear(&mut self) -> () {
        self.index.clear();
    }
}