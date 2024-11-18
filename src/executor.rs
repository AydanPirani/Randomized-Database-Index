use protobuf::Message;

use crate::database::{self, Database};
use crate::indexes::abstract_index::Index;
use crate::protos::operation::op::Operation;
use crate::types::{KeyT, ValT};
use crate::protos::operation::{self, Op, ReadOp, WriteOp};
use core::panic;
use std::collections::HashMap;
// use crate::protos::operation::Op
use std::io;
use std::fs::File;
use std::io::prelude::*;

use std::time::{Duration, Instant};
use std::thread::sleep;

pub struct SequenceExecutor {
    databases: Vec<Box<Database>>,
    ground_truth: HashMap<KeyT, ValT>,
    put_file: File,
    get_file: File,
}

impl SequenceExecutor {
    pub fn new() -> Self {
        SequenceExecutor {
            databases: vec![],
            ground_truth: HashMap::new(),
            get_file: File::create("get_output.txt").expect("Failed to open get_output.txt"),
            put_file: File::create("put_output.txt").expect("Failed to open put_output.txt"),

        }
    }

    pub fn add_index<T>(&mut self) 
    where 
        T: Index + 'static,
    {
        let index = T::new();
        let index_box = Box::new(index);
        let new_database = Box::new(Database::new(index_box));
        self.databases.push(new_database);
    }

    fn _execute_op(&mut self, op: Op) {
        match op.operation {
            Some (Operation::Read(ReadOp { key, special_fields: _ })) => {
                let start_time = Instant::now();
                self.expect(key);
                let elapsed_time = start_time.elapsed().as_nanos();
                let _ = self.get_file.write(format!("{}\n", elapsed_time).as_bytes());
            }
            Some (Operation::Write(WriteOp { key, value, special_fields: _ })) => {
                let start_time = Instant::now();
                self.insert(key, value);
                let elapsed_time = start_time.elapsed().as_nanos();
                let _ = self.put_file.write(format!("{}\n", elapsed_time).as_bytes());
            }
            _ => {
                panic!{"Something went wrong!"}
            }
        };

    }

    pub fn execute(&mut self, operation_file: &str) -> io::Result<()> {
        let mut file = File::open(operation_file)?;
        loop {
            let mut operation_length = [0u8; 4];
           
            let bytes_read = file.read(&mut operation_length)?;
            
            if bytes_read == 0 {
               break; 
            }

            let operation_size = u32::from_le_bytes(operation_length) as usize;
            let mut operation_buffer = vec![0u8; operation_size];

            file.read_exact(&mut operation_buffer)?;
            
            let op = Op::parse_from_bytes(&operation_buffer)?;
            self._execute_op(op);
        }
        
        Ok(())
    }

    pub fn insert(&mut self, key: KeyT, val: ValT) -> () {
        self.ground_truth.insert(key, val);
        for database in self.databases.iter_mut() {
            database.insert(key, val);
        }
    }

    // Function for testing purposes
    pub fn expect(&mut self, key: KeyT) -> bool {
        let exp_val = self.ground_truth.get(&key);
        for database in self.databases.iter_mut() {
            let query_result = database.get(&key);
            if query_result != exp_val {
                return false;
            }
        }

        return true;
    }
}
