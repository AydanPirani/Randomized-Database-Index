use crate::types::{KeyT, ValT};

pub trait Index {
    fn insert(&mut self, key: KeyT, val: ValT);
    fn get(&self, key: &KeyT) -> Option<&ValT>;
    fn remove(&mut self, key: &KeyT) -> Option<ValT>;
    fn contains_key(&self, key: &KeyT) -> bool;
    fn iter(&self) -> Box<dyn Iterator<Item = (&KeyT, &ValT)>>;
    fn clear(&mut self);
    fn len(&self) -> usize;
}