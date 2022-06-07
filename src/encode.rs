use crate::heap::*;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

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
        if (*parent_bit_1).0.len() > (*parent_bit_2).0.len() {
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
