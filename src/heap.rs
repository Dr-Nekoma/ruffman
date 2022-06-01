use ordered_float::OrderedFloat;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct HeapNode {
    pub probability: OrderedFloat<f64>,
    pub symbol: String,
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
