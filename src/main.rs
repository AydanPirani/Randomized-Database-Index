mod database;
mod indexes;
mod types;
// mod treap;

use database::Database;
use indexes::hashmap_index::HashMapIndex;
// use indexes::treap::TreapIndex;

fn main() {
    println!("Hello, world!");

    let hashmap_box = Box::new(HashMapIndex::new());

    let mut d = Database::new(hashmap_box);

    d.insert(12, 12);

    let rv = d.get(&12).unwrap();
    println!("{rv}");

    // Try out TreapIndex
    // let treap_box = Box::new(TreapIndex::new());
    // let mut d2 = Database::new(treap_box);
    // d2.insert(12, 12);
    // let rv2 = d2.get(&12).unwrap();
    // println!("{rv2}");
}
