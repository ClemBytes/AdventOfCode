use std::{collections::HashMap, fs};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY17 -------");
    let mut example: Vec<u32> = vec![20, 15, 10, 5, 5];

    let input = fs::read_to_string("inputs/input_day17").expect("Unable to read input!");
    let mut input = parse(&input);

    day17_part1(&mut example, &mut input);
    day17_part2(&mut example, &mut input);
}

const EGGNOG_EXAMPLE: u32 = 25;
const EGGNOG_INPUT: u32 = 150;

fn parse(raw_input: &str) -> Vec<u32> {
    let mut containers: Vec<u32> = vec![];
    for line in raw_input.lines() {
        containers.push(line.parse::<u32>().unwrap());
    }
    containers
}

fn recursive_nb_combinations_containers(
    containers_left: &mut Vec<u32>,
    total_in_containers: u32,
    total_eggnog: u32,
) -> u32 {
    if total_in_containers > total_eggnog {
        return 0;
    }
    if containers_left.is_empty() {
        if total_in_containers == total_eggnog {
            return 1;
        } else {
            return 0;
        }
    }
    let container = containers_left.pop().unwrap();
    let s = recursive_nb_combinations_containers(
        containers_left,
        total_in_containers + container,
        total_eggnog,
    ) + recursive_nb_combinations_containers(
        containers_left,
        total_in_containers,
        total_eggnog,
    );
    containers_left.push(container);
    s
}

fn recursive_min_combinations_containers(
    containers_left: &mut Vec<u32>,
    total_in_containers: u32,
    nb_containers: u32,
    total_eggnog: u32,
    results: &mut HashMap<u32, u32>, // key: nb_containers, value: nb_combinations for this nb of containers
) {
    if total_in_containers > total_eggnog {
        return;
    }
    if containers_left.is_empty() {
        if total_in_containers == total_eggnog {
            let count = results.entry(nb_containers).or_insert(0);
            *count += 1;
        }
        return;
    }

    let container = containers_left.pop().unwrap();

    // Use container
    recursive_min_combinations_containers(
        containers_left,
        total_in_containers + container,
        nb_containers + 1,
        total_eggnog,
        results,
    );

    // Don't use container
    recursive_min_combinations_containers(
        containers_left,
        total_in_containers,
        nb_containers,
        total_eggnog,
        results,
    );

    containers_left.push(container);
}

fn day17_part1(example: &mut Vec<u32>, input: &mut Vec<u32>) {
    println!("Example:  total: {EGGNOG_EXAMPLE:>3} | containers: {example:?}");
    println!("Input:    total: {EGGNOG_INPUT:>3} | containers: {input:?}");
    // Exemple tests
    assert_eq!(
        recursive_nb_combinations_containers(example, 0, EGGNOG_EXAMPLE),
        4
    );

    // Solve puzzle
    let res = recursive_nb_combinations_containers(input, 0, EGGNOG_INPUT);
    println!("Result part 1: {res}");
    assert_eq!(res, 4372);
    println!("> DAY17 - part 1: OK!");
}

fn solve_part2(input: &mut Vec<u32>, total_eggnog: u32) -> u32 {
    let mut res: HashMap<u32, u32> = HashMap::new();
    recursive_min_combinations_containers(input, 0, 0, total_eggnog, &mut res);
    let min_containers = res.keys().min().unwrap();
    res[min_containers]
}

fn day17_part2(example: &mut Vec<u32>, input: &mut Vec<u32>) {
    // Exemple tests
    let res = solve_part2(example, EGGNOG_EXAMPLE);
    println!("Res part2 example: {res}");
    assert_eq!(res, 3);

    // Solve puzzle
    let res = solve_part2(input, EGGNOG_INPUT);
    println!("Res part2: {res}");
    assert_eq!(res, 4);
    println!("> DAY17 - part 2: OK!");
}
