use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY11 -------");
    let example1 = fs::read_to_string("inputs/example_day11-1").expect("Unable to read input!");
    let example1 = parse(&example1);
    let example2 = fs::read_to_string("inputs/example_day11-2").expect("Unable to read input!");
    let example2 = parse(&example2);
    let input = fs::read_to_string("inputs/input_day11").expect("Unable to read input!");
    let input = parse(&input);

    day11_part1(&example1, &input);
    day11_part2(&example2, &input);
}

fn parse(raw_input: &str) -> HashMap<&str, Vec<&str>> {
    let mut connexions = HashMap::new();
    for line in raw_input.lines() {
        let (device, raw_outputs) = line.split_once(": ").unwrap();
        connexions.insert(device, raw_outputs.split_whitespace().collect());
    }
    connexions
}

fn find_all_paths_from_start_to_end(connexions: &HashMap<&str, Vec<&str>>, start: &str, end: &str) -> usize {
    // BFS without "seen_states"
    let mut nb_paths = 0;
    let mut devices_to_visit = VecDeque::new();
    devices_to_visit.push_back(start);
    while let Some(device) = devices_to_visit.pop_front() {
        if device == end {
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
    assert_eq!(find_all_paths_from_start_to_end(example, "you", "out"), 5);

    // Solve puzzle
    let res = find_all_paths_from_start_to_end(input, "you", "out");
    println!("Result part 1: {res}");
    assert_eq!(res, 615);
    println!("> DAY11 - part 1: OK!");
}

fn find_all_paths_from_svr_to_out_passing_by_dac_fft(connexions: &HashMap<&str, Vec<&str>>) -> usize {
    // Find all paths from svr to fft:
    let nb_path_svr_fft = find_all_paths_from_start_to_end(connexions, "svr", "fft");
    // Find all paths from fft to dac:
    let nb_path_fft_dac = find_all_paths_from_start_to_end(connexions, "fft", "dac");
    // Find all paths from dac to out:
    let nb_path_dac_out = find_all_paths_from_start_to_end(connexions, "dac", "out");

    // svr -> fft -> dac -> out
    let fft_then_dac = nb_path_svr_fft * nb_path_fft_dac * nb_path_dac_out;

    // Find all paths from svr to dac:
    let nb_path_svr_dac = find_all_paths_from_start_to_end(connexions, "svr", "dac");
    // Find all paths from dac to fft:
    let nb_path_dac_fft = find_all_paths_from_start_to_end(connexions, "dac", "fft");
    // Find all paths from fft to out:
    let nb_path_fft_out = find_all_paths_from_start_to_end(connexions, "fft", "out");

    // svr -> dac -> fft -> out
    let dac_then_fft = nb_path_svr_dac * nb_path_dac_fft * nb_path_fft_out;

    fft_then_dac + dac_then_fft
}

fn day11_part2(example: &HashMap<&str, Vec<&str>>, input: &HashMap<&str, Vec<&str>>) {
    // Exemple tests
    assert_eq!(find_all_paths_from_svr_to_out_passing_by_dac_fft(example), 2);
    println!("Example OK");

    // Solve puzzle
    let res = find_all_paths_from_svr_to_out_passing_by_dac_fft(input);
    println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY11 - part 2: OK!");
}
