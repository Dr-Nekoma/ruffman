use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn read_file(filename: &str, buffer: &mut Vec<u8>) -> io::Result<usize> {
    let mut f = File::open(filename).expect("File not found!");
    f.read_to_end(buffer)
}
