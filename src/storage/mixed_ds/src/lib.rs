use common::time::Duration;
use min_max_heap::MinMaxHeap;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};

pub struct Index {
    map: HashMap<String, Entry>,
}
impl Index {
    pub fn new() -> Index {
        let map: HashMap<String, Entry> = HashMap::with_capacity(1024);
        Index { map }
    }
    pub fn get(&self, key: &str) -> Option<&StrValue> {
        match self.map.get(key).map(|e| &e.value) {
            Some(Value::String { backing }) => Some(&backing),
            _ => None,
        }
    }

    pub fn set(&mut self, key: String, value: &StrValue) {
        let expiration = 0;
        let s = value.clone();
        self.map.insert(
            key,
            Entry {
                value: Value::String { backing: s },
                expiration,
            },
        );
    }
}
struct Entry {
    value: Value,
    expiration: u64,
}
pub enum Value {
    Map { backing: HashMap<String, StrValue> },
    List { backing: VecDeque<StrValue> },
    String { backing: StrValue },
}
#[derive(Clone, Eq, PartialEq)]
pub enum StrValue {
    Numeric(i64),
    StringValue(String),
}

// #[derive(Eq, PartialEq)]
// struct HeapElement<'a> {
//     score: i64,
//     element: StrValue<'a>,
// }
//
// impl PartialOrd for HeapElement {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         self.score.partial_cmp(&other.score)
//     }
// }
