mod database;
mod executor;
mod indexes;
mod logger;
mod types;

use executor::SequenceExecutor;
use indexes::hashmap_index::HashMapIndex;
use indexes::treap_index::TreapIndex;
use indexes::scapegoat_index::ScapegoatIndex;
use indexes::skiplist_index::SkipListIndex;
use indexes::splaytree_index::SplayTreeIndex;
use logger::Logger;

use std::env;

mod protos {
    include!(concat!(env!("OUT_DIR"), "/proto/mod.rs"));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Need 1) a file path to execute and 2) a file path to write to!");
    }

    let logger = Logger::new();
    let mut executor = SequenceExecutor::new(logger);

    // let hashmap = Box::new(HashMapIndex::new());
    // executor.add_index(hashmap, "hashmap");

    let skiplist = Box::new(SkipListIndex::new());
    executor.add_index(skiplist, "skiplist");

    let treap = Box::new(TreapIndex::new());
    executor.add_index(treap, "treap");

    let scapegoat = Box::new(ScapegoatIndex::new());
    executor.add_index(scapegoat, "scapegoat");

    let splaytree = Box::new(SplayTreeIndex::new());
    executor.add_index(splaytree, "splaytree");

    match executor.execute(&args[1], &args[2]) {
        Ok(_) => {
            println! {"Executed!"}
        }
        Err(_) => {
            panic!("Something went wrong!")
        }
    }
}
