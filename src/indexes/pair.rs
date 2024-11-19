use std::cmp::Ordering;
use crate::types::{ValT};

#[derive(Debug, Clone)]
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
