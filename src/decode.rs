use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn decompress_drn_file(
    buffer: &mut Vec<u8>,
    original_filename: &str,
) -> std::io::Result<String> {
    let n = original_filename.len();
    let mut new_filename = original_filename[0..(n - 4)].to_owned();
    new_filename += "_decompressed.txt";

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

    let mut decompressed_file = File::create(&new_filename)?;
    decompressed_file.write_all(&output)?;
    return Ok(new_filename);
}
