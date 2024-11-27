mod database;
mod executor;
mod indexes;
mod logger;
mod types;

use executor::SequenceExecutor;
use indexes::hashmap_index::HashMapIndex;
use indexes::treap_index::TreapIndex;
use indexes::treap_random_index::TreapRandomIndex;
use indexes::scapegoat_index::ScapegoatIndex;
use indexes::skiplist_index::SkipListIndex;
use indexes::splaytree_index::SplayTreeIndex;
use logger::Logger;

use std::env;

mod protos {
    include!(concat!(env!("OUT_DIR"), "/proto/mod.rs"));
}

const REPLICATION_FACTOR:u8 = 1;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Need 1) a file path to execute and 2) a file path to write to!");
    }

    let logger = Logger::new();
    let mut executor = SequenceExecutor::new(logger);

    executor.init_index("skiplist");
    executor.init_index("scapegoat");
    executor.init_index("splaytree");
    executor.init_index("treap");

    for _ in 0..REPLICATION_FACTOR {
        let skiplist = Box::new(SkipListIndex::new());
        executor.add_index("skiplist", skiplist);

        let scapegoat = Box::new(ScapegoatIndex::new());
        executor.add_index("scapegoat", scapegoat);

        let splaytree = Box::new(SplayTreeIndex::new());
        executor.add_index("splaytree", splaytree);
    
        let treap = Box::new(TreapIndex::new());
        executor.add_index("treap", treap);
    }


    match executor.execute(&args[1], &args[2]) {
        Ok(_) => {
            println! {"Executed!"}
        }
        Err(_) => {
            panic!("Something went wrong!")
        }
    }
}
