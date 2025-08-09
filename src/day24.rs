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
    let example: Vec<u64> = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
    let raw_input = fs::read_to_string("inputs/input_day24").expect("Unable to read input!");
    let mut input: Vec<u64> = vec![];
    for line in raw_input.lines() {
        input.push(line.parse::<u64>().unwrap());
    }

    day24_part1(&example, &input);
    day24_part2(&example, &input);
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct State {
    packages_to_fit: Vec<u64>,
    group1: Vec<u64>,
    others: Vec<u64>,
}

fn can_be_balanced_in_2_groups(mut list_of_packages: Vec<u64>) -> bool {
    let nb_packages = list_of_packages.len();
    match nb_packages {
        0 => false,
        1 => false,
        2 => list_of_packages[0] == list_of_packages[1],
        _ => {
            let last = list_of_packages.pop().unwrap();
            for i in 0..list_of_packages.len() {
                let mut list = list_of_packages.clone();
                list[i] += last;
                if can_be_balanced_in_2_groups(list) {
                    return true;
                }
            }
            false
        }
    }
}

#[test]
fn test_can_be_balanced_in_2_groups() {
    assert!(can_be_balanced_in_2_groups(vec![5, 5]));
    assert!(!can_be_balanced_in_2_groups(vec![2, 3]));
    assert!(can_be_balanced_in_2_groups(vec![2, 5, 3]));
    assert!(!can_be_balanced_in_2_groups(vec![2, 2, 1]));
    assert!(!can_be_balanced_in_2_groups(vec![]));
    assert!(!can_be_balanced_in_2_groups(vec![42]));
    assert!(can_be_balanced_in_2_groups(vec![3, 3, 3, 3]));
    assert!(can_be_balanced_in_2_groups(vec![10, 20, 15, 5]));
    assert!(!can_be_balanced_in_2_groups(vec![1, 1, 1]));
    assert!(!can_be_balanced_in_2_groups(vec![1, 2, 100]));
    assert!(can_be_balanced_in_2_groups(vec![0, 0, 5, 5]));
    assert!(!can_be_balanced_in_2_groups(vec![0, 1, 2]));
    assert!(!can_be_balanced_in_2_groups(vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9
    ]));
    assert!(can_be_balanced_in_2_groups(vec![
        1, 2, 3, 4, 6, 10, 8, 2, 4, 10
    ]));
    assert!(can_be_balanced_in_2_groups(vec![10; 20]));
    assert!(can_be_balanced_in_2_groups(vec![
        100, 200, 300, 400, 500, 600, 700, 800
    ]));
    assert!(!can_be_balanced_in_2_groups(vec![
        2, 4, 6, 8, 10, 12, 14, 16, 18
    ]));
}

impl State {
    fn from_packages(list_packages: &[u64]) -> Self {
        State {
            packages_to_fit: list_packages.to_owned(),
            group1: vec![],
            others: vec![],
        }
    }

    fn can_be_balanced(&self) -> bool {
        let weight_group1: u64 = self.group1.iter().sum();
        let weight_others: u64 = self.others.iter().sum();
        let weight_left: u64 = self.packages_to_fit.iter().sum();
        let total_weight: u64 = weight_group1 + weight_others + weight_left;
        if total_weight % 3 != 0 {
            return false;
        }
        let target_weight = total_weight / 3;
        if weight_group1 != target_weight {
            return false;
        }
        can_be_balanced_in_2_groups(self.others.clone())
    }

    fn next_states(&self) -> Vec<Self> {
        let mut next: Vec<State> = vec![];
        let next_state = &mut self.clone();
        let new_package = next_state.packages_to_fit.pop().unwrap();
        // Add it to group1
        next_state.group1.push(new_package);
        next.push(next_state.clone());
        // Take it off group1
        next_state.group1.pop();
        // Add it to others
        next_state.others.push(new_package);
        next.push(next_state.clone());
        next
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Other and self inversed to do a MIN-heap
        other.group1.len().cmp(&self.group1.len()).then(
            other
                .group1
                .iter()
                .product::<u64>()
                .cmp(&self.group1.iter().product::<u64>()),
        )
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_ideal_configuration(input: &[u64]) -> u64 {
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
            if current_state.can_be_balanced() {
                return current_state.group1.iter().product::<u64>();
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

fn day24_part1(example: &[u64], input: &[u64]) {
    // Exemple tests
    let res = find_ideal_configuration(example);
    println!("Result example part 1: {res}");
    assert_eq!(res, 99);

    // Solve puzzle
    let res = find_ideal_configuration(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 11266889531); // 22554787 is too low, 326440069283 is too high
    println!("> DAY24 - part 1: OK!");
}

fn day24_part2(_example: &[u64], _input: &[u64]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY24 - part 2: OK!");
}
