mod database;
mod indexes;
mod types;

use database::Database;
use indexes::hashmap_index::HashMapIndex;

fn main() {
    println!("Hello, world!");

    let hashmap_box = Box::new(HashMapIndex::new());

    let mut d = Database::new(hashmap_box);

    d.insert(12, 12);

    let rv = d.get(&12).unwrap();
    println!("{rv}");
}
