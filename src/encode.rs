use crate::heap::*;
use crate::io::*;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn calculate_probability(buffer: &mut Vec<u8>, total: usize) -> BinaryHeap<Reverse<HeapNode>> {
    let mut heap: BinaryHeap<Reverse<HeapNode>> = BinaryHeap::new();

    let unique_elements: Vec<u8> = buffer.clone().into_iter().unique().collect();

    for element in unique_elements.iter() {
        let node = HeapNode {
            probability: OrderedFloat(
                (buffer.iter().filter(|&n| *n == *element).count() as f64) / (total as f64),
            ),
            symbol: (*element).to_string(),
        };
        heap.push(Reverse(node));
    }

    return heap;
}

pub fn accumulate_hash_map(
    probabilities: &mut BinaryHeap<Reverse<HeapNode>>,
    map: &mut HashMap<String, (String, bool)>,
) {
    if probabilities.len() <= 2 {
        let potentially_last = probabilities.pop().unwrap();

        map.insert(potentially_last.0.symbol, (String::new(), true));

        if probabilities.len() == 1 {
            let last = probabilities.pop().unwrap();

            map.insert(last.0.symbol, (String::new(), false));
        }

        return;
    }

    let lowest = probabilities.pop().unwrap();
    let snd_lowest = probabilities.pop().unwrap();

    let mut new_symbol = lowest.0.symbol.clone();

    new_symbol.push_str(snd_lowest.0.symbol.as_str());

    map.insert(lowest.0.symbol, (new_symbol.clone(), true));

    map.insert(snd_lowest.0.symbol, (new_symbol.clone(), false));

    map.insert(new_symbol.clone(), ("".to_string(), false));

    let new_node = HeapNode {
        probability: lowest.0.probability + snd_lowest.0.probability,
        symbol: new_symbol,
    };

    probabilities.push(Reverse(new_node));

    return accumulate_hash_map(probabilities, map);
}

pub fn compare_huffman_nodes(
    t1: &(&String, &(String, bool)),
    t2: &(&String, &(String, bool)),
) -> std::cmp::Ordering {
    let (_, parent_bit_1) = *t1;
    let (_, parent_bit_2) = *t2;

    if (*parent_bit_1).0.len() == 0 {
        return Ordering::Less;
    } else if (*parent_bit_2).0.len() == 0 {
        return Ordering::Greater;
    } else {
        if (*parent_bit_1).0.len() < (*parent_bit_2).0.len() {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
}

pub fn bool_to_bit(b: bool) -> String {
    if b {
        return "1".to_string();
    } else {
        return "0".to_string();
    }
}

pub fn translate_symbols(map: &mut HashMap<String, (String, bool)>) -> HashMap<String, String> {
    let mut node_vector = Vec::from_iter(map.iter());
    node_vector.sort_by(|t1, t2| compare_huffman_nodes(t1, t2));

    let mut symbols_to_bits: HashMap<String, String> = HashMap::new();

    for node in node_vector {
        let (symbol, (parent, bool_bit)) = node;

        let bit = bool_to_bit(*bool_bit);

        if *parent == "" {
            symbols_to_bits.insert((*symbol).clone(), bit);
        } else {
            let parent_representation: String = symbols_to_bits[parent].clone();
            symbols_to_bits.insert(
                (*symbol).clone(),
                String::from(parent_representation.as_str().to_owned() + bit.as_str()),
            );
        }
    }

    return symbols_to_bits;
}

pub fn filter_by_symbols(symbols_to_maintain: Vec<String>, hash_map: &mut HashMap<String, String>) {
    for (symbol, _) in hash_map.clone().iter() {
        if !symbols_to_maintain.contains(symbol) {
            hash_map.remove_entry(symbol);
        }
    }
}

pub fn map_to_dict(content: &mut Vec<u8>, dictionary: &mut HashMap<String, String>) -> Vec<String> {
    return (content
        .iter()
        .map(|byte| dictionary[&(byte.to_string())].to_owned()))
    .collect();
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

    let translated_content: Vec<String> = map_to_dict(file_content, bit_representation);

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
