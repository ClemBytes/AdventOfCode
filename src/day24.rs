use std::cmp::Ordering;
use std::{
    collections::{BinaryHeap, HashSet},
    fs,
};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY24 -------");
    let example: Vec<u32> = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
    let raw_input = fs::read_to_string("inputs/input_day24").expect("Unable to read input!");
    let mut input: Vec<u32> = vec![];
    for line in raw_input.lines() {
        input.push(line.parse::<u32>().unwrap());
    }

    day24_part1(&example, &input);
    day24_part2(&example, &input);
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct State {
    packages_to_fit: Vec<u32>,
    len_group1: u32,
    weight_group1: u32,
    weight_group2: u32,
    weight_group3: u32,
    quantum_entanglement: u32,
}

impl State {
    fn from_packages(list_packages: &Vec<u32>) -> Self {
        State {
            packages_to_fit: list_packages.clone(),
            len_group1: 0,
            weight_group1: 0,
            weight_group2: 0,
            weight_group3: 0,
            quantum_entanglement: 1,
        }
    }

    fn next_states(&self) -> Vec<Self> {
        let mut next: Vec<State> = vec![];
        let next_state = &mut self.clone();
        let new_package = next_state.packages_to_fit.pop().unwrap();
        // Add it to group1
        next_state.len_group1 += 1;
        next_state.weight_group1 += new_package;
        next_state.quantum_entanglement *= new_package;
        next.push(next_state.clone());
        // Take it off group1
        next_state.len_group1 -= 1;
        next_state.weight_group1 -= new_package;
        next_state.quantum_entanglement /= new_package;
        // Add it to group2
        next_state.weight_group2 += new_package;
        next.push(next_state.clone());
        // Take it off group2
        next_state.weight_group2 -= new_package;
        // Add it to group3
        next_state.weight_group3 += new_package;
        next.push(next_state.clone());
        next
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Other and self inversed to do a MIN-heap
        other
            .len_group1
            .cmp(&self.len_group1)
            .then(other.quantum_entanglement.cmp(&self.quantum_entanglement))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_ideal_configuration(input: &Vec<u32>) -> u32 {
    let initial_state = State::from_packages(input);
    let mut seen_states: HashSet<State> = HashSet::new();
    let mut min_heap: BinaryHeap<State> = BinaryHeap::new();
    min_heap.push(initial_state);
    while let Some(current_state) = min_heap.pop() {
        // State already visited?
        if seen_states.contains(&current_state) {
            continue;
        }
        seen_states.insert(current_state.clone());

        // Finish?
        if current_state.packages_to_fit.is_empty() {
            // Found solution?
            if current_state.weight_group1 == current_state.weight_group2 && current_state.weight_group2 == current_state.weight_group3 {
                return current_state.quantum_entanglement;
            }
            // Not a solution:
            continue;
        }

        // Else: add neighbors to heap
        for next_state in current_state.next_states() {
            min_heap.push(next_state);
        }
    }
    unreachable!();
}

fn day24_part1(example: &Vec<u32>, input: &Vec<u32>) {
    // Exemple tests
    let res = find_ideal_configuration(example);
    println!("Result example part 1: {res}");
    assert_eq!(res, 99);

    // Solve puzzle
    let res = find_ideal_configuration(input);
    println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY24 - part 1: OK!");
}

fn day24_part2(_example: &Vec<u32>, _input: &Vec<u32>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY24 - part 2: OK!");
}
