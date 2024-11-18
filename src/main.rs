mod types;
mod indexes;
mod database;
mod executor;

use database::Database;
use executor::SequenceExecutor;
use indexes::hashmap_index::HashMapIndex;

use std::env;

mod protos {
    include!(concat!(env!("OUT_DIR"), "/proto/mod.rs"));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        panic!("Need a file path to execute!");
    }

    let index = HashMapIndex::new();
    let index_box = Box::new(index);

    let database = Database::new(index_box);
    let mut executor = SequenceExecutor::new(database);

    match executor.execute(&args[1]) {
        Ok (_) => {println!{"Executed!"}}
        Err (_) => {panic!("Something went wrong!")}
    }
}
