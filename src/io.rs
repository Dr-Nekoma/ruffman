#[path = "./encode.rs"]
pub mod encode;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use itertools::Itertools;

#[derive(Debug)]
pub struct HeaderCell {
    pub symbol: u8,
    pub size_of_bits: u8,
    pub bits: String,
}

pub fn read_file(filename: &str, buffer: &mut Vec<u8>) -> io::Result<usize> {
    let mut f = File::open(filename).expect("File not found!");
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

pub fn compress_file(
    filename: &str,
    file_content: &mut Vec<u8>,
    bit_representation: &mut HashMap<String, String>,
) -> std::io::Result<()> {
    let n = filename.len();

    let mut new_filename = filename[0..(n - 4)].to_owned();
    new_filename += ".drn";

    let header_cells: Vec<HeaderCell> = get_dictionary_header(bit_representation);

    let mut bits: String = String::new();

    bits.push_str(format!("{:08b}", header_cells.len()).as_str());

    for cell in header_cells.iter() {
        bits.push_str(format!("{:08b}", cell.symbol).as_str());
        bits.push_str(format!("{:08b}", cell.size_of_bits).as_str());
        bits.push_str(cell.bits.as_str());
    }

    let translated_content: Vec<String> = encode::map_to_dict(file_content, bit_representation);

    let padding = get_padding_bits(header_cells, &translated_content);

    bits += format!("{:08b}", padding).as_str();

    for content in translated_content.iter() {
        bits += content.as_str();
    }

    for _ in 0..padding {
        bits += "0";
    }

    let mut index = 0;
    let mut binary_vector: Vec<u8> = Vec::new();

    while index < bits.len() {
        let byte = u8::from_str_radix(&(bits.as_str())[index..index + 8], 2)
            .expect("Not a binary number!");
        binary_vector.push(byte);
        index += 8;
    }

    let mut compressed_file = File::create(new_filename)?;
    compressed_file.write_all(&binary_vector)?;
    return Ok(());
}

pub fn read_drn_file(original_filename: &str, filename: &str) -> std::io::Result<()> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut f = File::open(filename).expect("File not found!");
    f.read_to_end(&mut buffer).expect("Error reading the file!");

    let how_many_symbols = buffer[0].to_owned();

    let mut stream_of_bits: String = buffer.iter().map(|byte| format!("{:08b}", byte)).join("");

    let mut symbol_hash: HashMap<String, u8> = HashMap::new();

    let mut index_for_bits = 8;
    let mut index_for_symbols = 0;

    while index_for_symbols < how_many_symbols {
        let symbol: u8 =
            u8::from_str_radix(&stream_of_bits[index_for_bits..(index_for_bits + 8)], 2)
                .expect("Not a binary number!");
        let size: u8 = u8::from_str_radix(
            &stream_of_bits[(index_for_bits + 8)..(index_for_bits + 16)],
            2,
        )
        .expect("Not a binary number!");
        let bit_representation: String = stream_of_bits
            [(index_for_bits + 16)..(index_for_bits + 16 + size as usize)]
            .to_string();
        index_for_bits += 16 + size as usize;
        index_for_symbols += 1;
        symbol_hash.insert(bit_representation, symbol);
    }

    let padding: u8 = u8::from_str_radix(&stream_of_bits[index_for_bits..index_for_bits + 8], 2)
        .expect("Not a binary number!");

    if padding != 0 {
        for _ in 0..padding {
            stream_of_bits.pop();
        }
    }

    index_for_bits += 8;
    //let slice: String = stream_of_bits.chars().skip(index_for_bits).collect();

    let mut output: Vec<u8> = Vec::new();

    while index_for_bits <= stream_of_bits.len() - 1 {
        let mut byte: u8 = stream_of_bits.as_bytes()[index_for_bits];
        let mut c: char = byte as char;

        let mut bits = String::new();
        bits.push(c);

        let mut attempt_of_symbol = symbol_hash.get(&bits.to_string());
        while attempt_of_symbol.is_none() {
            index_for_bits += 1;

            byte = stream_of_bits.as_bytes()[index_for_bits];
            c = byte as char;
            bits.push(c);

            attempt_of_symbol = symbol_hash.get(&bits);
        }
        index_for_bits += 1;
        output.push(*attempt_of_symbol.unwrap());
        bits.clear();
    }
    let mut decompressed_file = File::create("thisIsATest.txt")?;
    decompressed_file.write_all(&output)?;
    return Ok(());
}
