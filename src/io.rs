#[path = "./encode.rs"]
mod encode;
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

pub fn read_file(filename: &str, buffer: &mut Vec<u8>) -> io::Result<usize> {
    let mut f = File::open(filename).expect("File not found!");
    f.read_to_end(buffer)
}

pub fn get_dictionary_header(bit_representation: &mut HashMap<String, String>) -> Vec<HeaderCell> {
    let mut dict_header: Vec<String> = Vec::new();
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
    let reminder = ((header_bit_size as i32 + content_bit_size as i32) % 8);
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
) {
    let n = filename.len();

    let mut new_filename = filename[0..(n - 4)].to_owned();
    new_filename += ".drn";

    let header_cells: Vec<HeaderCell> = get_dictionary_header(bit_representation);
    let mut bits: String = String::new();

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

    for index in 0..padding {
        bits += "0";
    }

    // TODO: Write this into a file

    //let mut compressed_file = File::create(new_filename)?;

    //compressed_file.write(buf)
}
