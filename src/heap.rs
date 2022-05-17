use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Eq)]
pub struct HeapNode {
    probability: OrderedFloat<f64>,
    symbol: u8,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.probability.cmp(&other.probability)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.probability.eq(&other.probability) && self.symbol == other.symbol
    }
}

pub fn calculate_probability(buffer: &mut Vec<u8>, total: usize) -> BinaryHeap<HeapNode> {
    let mut heap: BinaryHeap<HeapNode> = BinaryHeap::new();

    let unique_elements: Vec<u8> = buffer.clone().into_iter().unique().collect();

    for element in unique_elements.iter() {
        let node = HeapNode {
            probability: OrderedFloat(
                (buffer.iter().filter(|&n| *n == *element).count() as f64) / (total as f64),
            ),
            symbol: *element,
        };
        heap.push(node);
    }

    return heap;
}
