use crate::types::{KeyT, ValT};

use teardown_tree___treap as treap;
use treap::{TreapMap};
use rand::{XorShiftRng};

use super::abstract_index::Index;

pub struct TreapRandomIndex {
    index: TreapMap<KeyT, ValT, XorShiftRng>,
}

impl TreapRandomIndex {
    pub fn new() -> Self {
        TreapRandomIndex {
            index: TreapMap::new(),
        }
    }
}

impl Index for TreapRandomIndex {
    fn insert(&mut self, key: KeyT, val: ValT) {
        if let Some(element) = self.index.get(&key)  {
            self.index.remove(&key);
            self.index.insert(key, val);
        }
        else {
            self.index.insert(key, val);
        }
    }

    fn get(&mut self, key: &KeyT) -> Option<&ValT> {
        return self.index.get(key);
    }

    fn clear(&mut self) -> () {
        self.index.clear();
    }
}