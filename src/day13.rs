use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY13 -------");
    let example = fs::read_to_string("inputs/example_day13").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day13").expect("Unable to read input!");
    let input = parse(&input);

    day13_part1(&example, &input);
    day13_part2(&input);
}

fn parse(raw_input: &str) -> HashMap<&str, HashMap<&str, i32>> {
    let mut guests_list: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    for line in raw_input.lines() {
        // First split: name
        let split: Vec<&str> = line.split(" would ").collect();
        let name = split[0];
        let rest = split[1];
        // Second split: "gain/lose XX" and relation name
        let split: Vec<&str> = rest.split(" happiness units by sitting next to ").collect();
        let happiness_change = split[0];
        let relation_name = split[1];
        // Third split: analyze gain or loss
        let split: Vec<&str> = happiness_change.split(" ").collect();
        let mut happiness_change: i32 = split[1].parse().unwrap();
        if split[0] == "lose" {
            happiness_change = -happiness_change;
        }

        guests_list
            .entry(name)
            .or_default()
            .insert(relation_name, happiness_change);
    }
    guests_list
}

fn get_mutual_happiness(
    guest1: &str,
    guest2: &str,
    relations: &HashMap<&str, HashMap<&str, i32>>,
) -> i32 {
    if guest1 == "Clementine" || guest2 == "Clementine" {
        0
    } else {
        relations.get(&guest1).unwrap().get(&guest2).unwrap()
            + relations.get(&guest2).unwrap().get(&guest1).unwrap()
    }
}

fn recursive_search_max(
    relations: &HashMap<&str, HashMap<&str, i32>>,
    guests_list: &mut VecDeque<&str>,
    current_guest: &str,
    first_guest: &str,
) -> i32 {
    if guests_list.is_empty() {
        // TODO: no return 0 but happiness with first_guess
        return get_mutual_happiness(current_guest, first_guest, relations);
    }
    let mut max_happiness = 0;
    let n = guests_list.len();
    for _ in 0..n {
        let guest = guests_list.pop_front().unwrap();
        let total_happiness = get_mutual_happiness(guest, current_guest, relations)
            + recursive_search_max(relations, guests_list, guest, first_guest);
        if total_happiness > max_happiness {
            max_happiness = total_happiness;
        }
        guests_list.push_back(guest);
    }
    max_happiness
}

fn solve_part1(input: &HashMap<&str, HashMap<&str, i32>>) -> i32 {
    let mut guests_list: VecDeque<&str> = input.keys().copied().collect();
    let first_guest = guests_list.pop_front().unwrap();
    recursive_search_max(input, &mut guests_list, first_guest, first_guest)
}

fn day13_part1(
    example: &HashMap<&str, HashMap<&str, i32>>,
    input: &HashMap<&str, HashMap<&str, i32>>,
) {
    // Exemple tests
    println!("Example: {example:#?}");
    assert_eq!(solve_part1(example), 330);

    // Solve puzzle
    let res = solve_part1(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 664);
    println!("> DAY13 - part 1: OK!");
}

fn solve_part2(input: &HashMap<&str, HashMap<&str, i32>>) -> i32 {
    let mut guests_list: VecDeque<&str> = input.keys().copied().collect();
    recursive_search_max(input, &mut guests_list, "Clementine", "Clementine")
}

fn day13_part2(input: &HashMap<&str, HashMap<&str, i32>>) {
    // Solve puzzle
    let res = solve_part2(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 640);
    println!("> DAY13 - part 2: OK!");
}
