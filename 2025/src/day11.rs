use std::{collections::{HashMap, VecDeque}, fs};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY11 -------");
    let example = fs::read_to_string("inputs/example_day11").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day11").expect("Unable to read input!");
    let input = parse(&input);

    day11_part1(&example, &input);
    day11_part2(&example, &input);
}

fn parse(raw_input: &str) -> HashMap<&str, Vec<&str>> {
    let mut connexions = HashMap::new();
    for line in raw_input.lines() {
        let (device, raw_outputs) = line.split_once(": ").unwrap();
        connexions.insert(device, raw_outputs.split_whitespace().collect());
    }
    connexions
}

fn find_all_paths_from_you_to_out(connexions: &HashMap<&str, Vec<&str>>) -> usize {
    // BFS without "seen_states"
    let mut nb_paths = 0;
    let mut devices_to_visit = VecDeque::new();
    devices_to_visit.push_back("you");
    while let Some(device) = devices_to_visit.pop_front() {
        if device == "out" {
            nb_paths += 1;
            continue;
        }

        if connexions.contains_key(device) {
            let outputs = connexions.get(device).unwrap().clone();
            for output in outputs {
                devices_to_visit.push_back(output);
            }
        }
    }
    nb_paths
}

fn day11_part1(example: &HashMap<&str, Vec<&str>>, input: &HashMap<&str, Vec<&str>>) {
    // Exemple tests
    assert_eq!(find_all_paths_from_you_to_out(example), 5);

    // Solve puzzle
    let res = find_all_paths_from_you_to_out(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 615);
    println!("> DAY11 - part 1: OK!");
}

fn day11_part2(_example: &HashMap<&str, Vec<&str>>, _input: &HashMap<&str, Vec<&str>>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY11 - part 2: OK!");
}
