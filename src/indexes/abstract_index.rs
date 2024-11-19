use crate::types::{KeyT, ValT};

pub trait Index {
    fn insert(&mut self, key: KeyT, val: ValT) -> ();
    fn get(&mut self, key: &KeyT) -> Option<&ValT>;
}