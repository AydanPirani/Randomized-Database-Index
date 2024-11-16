mod types;
mod indexes;
mod database;
mod executor;

use database::Database;
use executor::SequenceExecutor;
use indexes::hashmap_index::HashMapIndex;

mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

fn main() {
    let index = HashMapIndex::new();
    let index_box = Box::new(index);

    let database = Database::new(index_box);
    let mut executor = SequenceExecutor::new(database);

    executor.insert(12, 12);
    let rv = executor.get(12).unwrap();
    println!("{rv}")
}
