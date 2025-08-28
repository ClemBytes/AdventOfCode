use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[test]
fn test() {
    run();
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    mana: i32,
    name: String,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Other and self inversed to do a MIN-heap
        other.mana.cmp(&self.mana)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run() {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State {
        mana: 50,
        name: String::from("Bla"),
    });
    heap.push(State {
        mana: 100,
        name: String::from("Bli"),
    });
    heap.push(State {
        mana: 25,
        name: String::from("Blo"),
    });
    while !heap.is_empty() {
        println!("{:?}", heap.pop().unwrap());
    }
}
