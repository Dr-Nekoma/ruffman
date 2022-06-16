use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub struct HeaderCell {
    pub symbol: u8,
    pub size_of_bits: u8,
    pub bits: String,
}

pub fn read_file(file_name: &str, buffer: &mut Vec<u8>) -> io::Result<usize> {
    let mut f = File::open(file_name).expect("File not found!");
    f.read_to_end(buffer)
}

pub fn get_dictionary_header(bit_representation: &mut HashMap<String, String>) -> Vec<HeaderCell> {
    return bit_representation
        .iter()
        .map(|(symbol, bits)| HeaderCell {
            symbol: symbol.parse().unwrap(),
            size_of_bits: bits.len() as u8,
            bits: bits.to_owned(),
        })
        .collect();
}

pub fn get_padding_bits(header: Vec<HeaderCell>, mapped_content: &Vec<String>) -> i32 {
    let header_bit_size = header
        .iter()
        .fold(0, |acc, cell| acc + 8 + 8 + cell.bits.len());
    let content_bit_size = mapped_content.iter().fold(0, |acc, bits| acc + bits.len());
    let reminder = (header_bit_size as i32 + content_bit_size as i32) % 8;
    if reminder == 0 {
        return 0;
    } else {
        return 8 - reminder;
    }
}

pub fn check_original_and_decompressed_files(file_name: &str, decompressed_file_name: &str) {
    let mut file_content: Vec<u8> = Vec::new();
    let mut decoded_file_content: Vec<u8> = Vec::new();

    read_file(file_name, &mut file_content).unwrap();
    read_file(decompressed_file_name, &mut decoded_file_content).unwrap();

    if file_content == decoded_file_content {
        println!("The files (original and decompressed) are equal!");
    } else {
        println!("The files (original and decompressed) are not equal!");
    }
}

pub fn compare_file_sizes(original_file_name: &str, compressed_file_name: &str) {
    let mut file_content: Vec<u8> = Vec::new();
    let mut encoded_file_content: Vec<u8> = Vec::new();

    read_file(original_file_name, &mut file_content).unwrap();
    read_file(compressed_file_name, &mut encoded_file_content).unwrap();

    println!(
        "\nCompression Rate: {}%",
        (100.0 - (encoded_file_content.len() as f64 / file_content.len() as f64) * 100.0)
    );
}
