mod encode;
mod heap;
mod io;
use std::{collections::HashMap};

fn main() {
    let file_name = "test.txt";

    let mut file_content: Vec<u8> = Vec::new();

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

    println!("{:?}", bit_representation);
}
