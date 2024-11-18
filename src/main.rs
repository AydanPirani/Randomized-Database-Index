mod types;
mod indexes;
mod database;
mod executor;

use database::Database;
use executor::SequenceExecutor;
use indexes::hashmap_index::HashMapIndex;
use indexes::skiplist_index::SkipListIndex;

use std::env;

mod protos {
    include!(concat!(env!("OUT_DIR"), "/proto/mod.rs"));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        panic!("Need a file path to execute!");
    }

    // let index = HashMapIndex::new();
    // let index = SkipListIndex::new();
    // let index_box = Box::new(index);

    let mut executor = SequenceExecutor::new();
    executor.add_index::<SkipListIndex>();

    match executor.execute(&args[1]) {
        Ok (_) => {println!{"Executed!"}}
        Err (_) => {panic!("Something went wrong!")}
    }
}
