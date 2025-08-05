use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY17 -------");
    let mut example: Vec<u32> = vec![];
    example.push(20);
    example.push(15);
    example.push(10);
    example.push(5);
    example.push(5);

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

fn recursive_nb_combinations_containers(containers_left: &mut Vec<u32>, total_containers: u32, total_eggnog: u32) -> u32 {
    if total_containers > total_eggnog {
        return 0;
    }
    if containers_left.is_empty() {
        if total_containers == total_eggnog {
            return 1;
        } else {
            return 0;
        }
    }
    let container = containers_left.pop().unwrap();
    let s = recursive_nb_combinations_containers(containers_left, total_containers + container, total_eggnog)
        + recursive_nb_combinations_containers(containers_left, total_containers, total_eggnog);
    containers_left.push(container);
    s
}

fn day17_part1(example: &mut Vec<u32>, input: &mut Vec<u32>) {
    println!("Example:  total: {EGGNOG_EXAMPLE:>3} | containers: {example:?}");
    println!("Input:    total: {EGGNOG_INPUT:>3} | containers: {input:?}");
    // Exemple tests
    assert_eq!(recursive_nb_combinations_containers(example, 0, EGGNOG_EXAMPLE), 4);

    // Solve puzzle
    let res = recursive_nb_combinations_containers(input, 0, EGGNOG_INPUT);
    println!("Result part 1: {res}");
    assert_eq!(res, 4372);
    println!("> DAY17 - part 1: OK!");
}

fn day17_part2(_example: &mut Vec<u32>, _input: &mut Vec<u32>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // println!("Result part 2: {}");
    // assert_eq!(, );
    // println!("> DAY17 - part 2: OK!");
}
