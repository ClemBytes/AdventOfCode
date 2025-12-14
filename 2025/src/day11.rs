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

fn bfs_you_to_out(connexions: &HashMap<&str, Vec<&str>>) -> usize {
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
    assert_eq!(bfs_you_to_out(example), 5);

    // Solve puzzle
    let res = bfs_you_to_out(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 615);
    println!("> DAY11 - part 1: OK!");
}

fn find_all_paths_from_svr_to_out_passing_by_dac_fft(
    connexions: &HashMap<&str, Vec<&str>>,
) -> usize {
    // Let's try recursive DFS with add memoization:
    // For the memoization, we want to remember:
    // “I’ve already reached this node with this state → here’s the result of exploring the rest of the graph from here with this state.”
    fn recursive_dfs<'a>(
        current_device: &str,
        visited_dac: bool,
        visited_fft: bool,
        connexions: &HashMap<&str, Vec<&'a str>>,
        memo: &mut HashMap<(&'a str, bool, bool), usize>,
    ) -> usize {
        // Check final condition
        if current_device == "out" {
            if visited_dac && visited_fft {
                return 1;
            } else {
                return 0;
            }
        }

        let mut nb_paths = 0;
        let next_visited_dac = visited_dac || (current_device == "dac");
        let next_visited_fft = visited_fft || (current_device == "fft");

        // Now check each children
        for &next_device in connexions.get(current_device).unwrap() {
            // If already in memoization, use it:
            if memo.contains_key(&(next_device, next_visited_dac, next_visited_fft)) {
                nb_paths += memo
                    .get(&(next_device, next_visited_dac, next_visited_fft))
                    .unwrap();
            } else {
                // If not, then recursion and add it to memo
                let nb_paths_next_state = recursive_dfs(
                    next_device,
                    next_visited_dac,
                    next_visited_fft,
                    connexions,
                    memo,
                );
                nb_paths += nb_paths_next_state;
                memo.insert(
                    (next_device, next_visited_dac, next_visited_fft),
                    nb_paths_next_state,
                );
            }
        }

        nb_paths
    }
    let mut memo = HashMap::new();
    recursive_dfs("svr", false, false, connexions, &mut memo)
}

fn day11_part2(example: &HashMap<&str, Vec<&str>>, input: &HashMap<&str, Vec<&str>>) {
    // Exemple tests
    assert_eq!(
        find_all_paths_from_svr_to_out_passing_by_dac_fft(example),
        2
    );

    // Solve puzzle
    let res = find_all_paths_from_svr_to_out_passing_by_dac_fft(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 303012373210128);
    println!("> DAY11 - part 2: OK!");
}
