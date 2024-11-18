mod database;
mod indexes;
mod types;

use indexes::scapegoat_index::ScapegoatIndex;
use indexes::abstract_index::Index; // Import the Index trait
use types::{KeyT, ValT};

fn main() {
    // Test insert and get
    let mut index = ScapegoatIndex::new();
    index.insert(1, 10);
    println!("Inserted (1, 10)");
    assert_eq!(index.get(&1), Some(&10));
    println!("Retrieved value for key 1: {:?}", index.get(&1));

    // Test remove
    index.insert(1, 10);
    println!("Inserted (1, 10)");
    assert_eq!(index.remove(&1), Some(10));
    println!("Removed key 1, value was 10");
    assert_eq!(index.get(&1), None);
    println!("Checked that key 1 is no longer present");

    // Test contains_key
    index.insert(1, 10);
    println!("Inserted (1, 10)");
    assert!(index.contains_key(&1));
    println!("Key 1 is present");
    assert!(!index.contains_key(&2));
    println!("Key 2 is not present");

    // Test iter
    index.insert(1, 10);
    index.insert(2, 20);
    println!("Inserted (1, 10) and (2, 20)");
    {
        let mut iter = index.iter();
        println!("Iterating over elements:");
        assert_eq!(iter.next(), Some((&1, &10)));
        println!("Found (1, 10)");
        assert_eq!(iter.next(), Some((&2, &20)));
        println!("Found (2, 20)");
        assert_eq!(iter.next(), None);
        println!("No more elements");
    } // Ensure the iterator is dropped here

    // Test clear
    index.insert(1, 10);
    index.insert(2, 20);
    println!("Inserted (1, 10) and (2, 20)");
    index.clear();
    println!("Cleared all elements");
    assert_eq!(index.len(), 0);
    println!("Length is 0");
    assert!(!index.contains_key(&1));
    println!("Key 1 is not present");
    assert!(!index.contains_key(&2));
    println!("Key 2 is not present");

    // Test len
    assert_eq!(index.len(), 0);
    println!("Initial length is 0");
    index.insert(1, 10);
    println!("Inserted (1, 10)");
    assert_eq!(index.len(), 1);
    println!("Length is 1");
    index.insert(2, 20);
    println!("Inserted (2, 20)");
    assert_eq!(index.len(), 2);
    println!("Length is 2");
    index.remove(&1);
    println!("Removed key 1");
    assert_eq!(index.len(), 1);
    println!("Length is 1");
}