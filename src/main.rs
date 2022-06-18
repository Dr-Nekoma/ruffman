pub mod decode;
pub mod encode;
pub mod heap;
pub mod io;
use std::collections::HashMap;
use std::env;
use std::process;

pub fn main() {
    let file_name = match env::args().nth(1) {
        Some(str) => {
            println!("\nFile path provided: {}", str);
            str
        }
        None => {
            println!("\nFile path not provided!");
            process::exit(1);
        }
    };

    let file_name: &str = file_name.as_str();

    let mut file_content: Vec<u8> = Vec::new();

    // ENCODING

    let number_elements = io::read_file(file_name, &mut file_content).unwrap();

    let mut probabilities = encode::calculate_probability(&mut file_content, number_elements);

    let mut original_symbols: Vec<String> = Vec::new();

    for node in probabilities.iter() {
        original_symbols.push(node.0.symbol.clone());
    }

    let mut huffman_map: HashMap<String, (String, bool)> = HashMap::new();

    encode::accumulate_hash_map(&mut probabilities, &mut huffman_map);

    let mut bit_representation: HashMap<String, String> =
        encode::translate_symbols(&mut huffman_map);

    encode::filter_by_symbols(original_symbols, &mut bit_representation);

    let encode_result: std::io::Result<()> =
        encode::compress_file(file_name, &mut file_content, &mut bit_representation);

    match encode_result {
        Ok(()) => println!("\nEncoded with success!"),
        _ => println!("\nError during enconding!"),
    }

    let mut encoded_file_content: Vec<u8> = Vec::new();

    let n = file_name.len();
    let mut encoded_file_name = file_name[0..(n - 4)].to_owned();
    encoded_file_name += ".drn";

    io::compare_file_sizes(file_name, &encoded_file_name);

    // DECODING

    io::read_file(encoded_file_name.as_str(), &mut encoded_file_content).unwrap();

    let decoded_result: std::io::Result<String> =
        decode::decompress_drn_file(&mut encoded_file_content, file_name);

    match decoded_result {
        Ok(str) => {
            println!("\nDecoded with success!\n");
            io::check_original_and_decompressed_files(file_name, str.as_str())
        }
        _ => println!("\nError during deconding!"),
    }
}
