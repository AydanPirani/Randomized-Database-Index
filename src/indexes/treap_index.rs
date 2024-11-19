use std::cmp::Ordering;
use crate::types::{KeyT, ValT};

use treap_non_random as treap;
use treap::{Element, Treap};

use super::abstract_index::Index;

struct Pair {
    priority: u32,
    value: ValT,
}

// Implement the PartialEq and Eq traits
impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Pair {}

// Implement the PartialOrd and Ord traits for custom comparison logic
impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

pub struct TreapIndex {
    index: Treap<KeyT, Pair>,
}

impl TreapIndex {
    pub fn new() -> Self {
        TreapIndex {
            index: Treap::new(),
        }
    }
}

impl Index for TreapIndex {
    fn insert(&mut self, key: KeyT, val: ValT) {
        self.index.insert(Element::new(key, Pair { priority: 0, value: val}))
    }

    fn get(&mut self, key: &KeyT) -> Option<&ValT> {
        if let Some(element) = self.index.get(*key)  {
            let value = element.priority().value;
            let new_priority = element.priority().priority + 1;
            self.index.insert(Element::new(*key, Pair { priority: new_priority+1, value: value}));
        }  else {
            return None;
        }    

        return Some(&(self.index.get(*key)?.priority().value));
    }
}