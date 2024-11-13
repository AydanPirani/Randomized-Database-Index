use crate::indexes::abstract_index::Index;
use crate::types::{KeyT, ValT};

pub struct Database {
    index: Box<dyn Index>,
}

impl Database {
    pub fn new(index: Box<dyn Index>) -> Self {
        Database {
            index: index,
        }
    }

    pub fn insert(&mut self, key: KeyT, val: ValT) {
        self.index.insert(key, val);
    }

    pub fn get(&self, key: &KeyT) -> Option<&ValT> {
        return self.index.get(key);
    }
}
