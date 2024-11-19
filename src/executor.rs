use protobuf::Message;

use crate::database::{Database};
use crate::indexes::abstract_index::Index;
use crate::protos::operation::op::Operation;
use crate::types::{KeyT, ValT};
use crate::protos::operation::{Op, ReadOp, WriteOp};
use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

use crate::logger::Logger;

use std::time::Instant;

pub struct SequenceExecutor {
    databases: HashMap<&'static str, Database>,
    ground_truth: HashMap<KeyT, ValT>,
    logger: Box<Logger>
}

impl SequenceExecutor {
    pub fn new(logger: Logger) -> Self {
        SequenceExecutor {
            databases: HashMap::new(),
            ground_truth: HashMap::new(),
            logger: Box::new(logger),
        }
    }

    pub fn add_index(&mut self, index: Box<dyn Index>, index_name: &'static str) 
    {
        let database = Database::new(index);
        self.databases.insert(index_name, database);
    }

    fn _execute_op(&mut self, op: Op) {
        match op.operation {
            Some (Operation::Read(ReadOp { key, special_fields: _ })) => {
                self.expect(key);

            }
            Some (Operation::Write(WriteOp { key, value, special_fields: _ })) => {
                self.insert(key, value);
            }
            _ => {
                panic!{"Something went wrong!"}
            }
        };

    }

    pub fn execute(&mut self, operation_file: &str, output_filepath: &str) -> io::Result<()> {
        let mut file = File::open(operation_file)?;
        self.logger.init(output_filepath)?;
        self.clear();

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

    fn insert(&mut self, key: KeyT, val: ValT) -> () {
        self.ground_truth.insert(key, val);

        for (index_name, database) in self.databases.iter_mut() {
            let start_time = Instant::now();
            database.insert(key, val);
            let elapsed_time = start_time.elapsed().as_nanos();
            self.logger.write(index_name, "write", key, elapsed_time);
        }
    }

    fn expect(&mut self, key: KeyT) -> bool {
        let exp_val = self.ground_truth.get(&key);

        for (index_name, database) in self.databases.iter_mut() {
            let start_time = Instant::now();
            let query_result = database.get(&key);
            let elapsed_time = start_time.elapsed().as_nanos();

            if query_result != exp_val {
                panic!("{index_name} implemented incorrectly!")
            }
            self.logger.write(index_name, "read", key, elapsed_time);
        }

        return true;
    }

    fn clear(&mut self) -> () {
        for database in self.databases.values_mut() {
            database.clear();
        }
    }
}
