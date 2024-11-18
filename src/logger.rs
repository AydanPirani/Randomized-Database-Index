use std::fs::File;
use crate::types::{KeyT};
use std::io::prelude::*;
use std::io::{self, Write};

pub struct Logger {
    out_file: Option<File>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            out_file: None,
        }
    }

    pub fn init(&mut self, output_filepath: &str) -> io::Result<()> {
        let mut out_file = File::create(output_filepath)?;
        out_file.write(b"index,key,op,nanosecs\n")?;
        self.out_file = Some(out_file);
        return Ok(());
    }

    pub fn write(&self, index_name: &'static str, op_name: &'static str, key: KeyT, elapsed_time: u128) {
        let _ = self.out_file.as_ref().unwrap().write(format!("{},{},{},{}\n", index_name, op_name, key, elapsed_time).as_bytes());
    }
}