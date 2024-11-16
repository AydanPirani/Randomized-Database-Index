use protobuf::Message;

use crate::database::{self, Database};
use crate::protos::operation::op::Operation;
use crate::types::{KeyT, ValT};
use crate::protos::operation::{self, Op, ReadOp, WriteOp};
// use crate::protos::operation::Op
use std::io;
use std::fs::File;
use std::io::prelude::*;

pub struct SequenceExecutor {
    database: Box<Database>,
}

impl SequenceExecutor {
    pub fn new(database: Database) -> Self {
        SequenceExecutor {
            database: Box::new(database),
        }
    }

    fn _execute_op(&mut self, op: Op) {
        let f: Box<dyn FnOnce()> = match op.operation {
            Some (Operation::Read(ReadOp { key, special_fields: _ })) => {
                Box::new(move || { self.get(key); })
            }
            Some (Operation::Write(WriteOp { key, value, special_fields: _ })) => {
                Box::new(move || { self.insert(key, value); }) 
            }
            _ => {return;}
        };

        // Can add stuff here to perform timing/other metric collection
        f();
    }


    pub fn execute(&mut self, operation_file: &str) -> io::Result<()> {
        let mut file = File::open(operation_file)?;

        loop {
            let mut operation_length = [0u8; 4];
           
            file.read_exact(&mut operation_length)?;

            let operation_size = u32::from_le_bytes(operation_length) as usize;
            let mut operation_buffer = vec![0u8; operation_size];

            file.read_exact(&mut operation_buffer)?;
            
            let op = Op::parse_from_bytes(&operation_buffer)?;
            self._execute_op(op);
        }
    }

    pub fn insert(&mut self, key: KeyT, val: ValT) -> () {
        self.database.insert(key, val);
    }

    pub fn get(&mut self, key: KeyT) -> Option<&ValT> {
        return self.database.get(&key);
    }

    // Function for testing purposes
    pub fn expect(&mut self, key: KeyT, expVal: ValT) -> bool {
        let query_result = self.database.get(&key);
        match query_result {
            Some(val) => return *val == expVal,
            None => return false,
        }
    }
}
