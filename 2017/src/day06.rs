use std::{collections::HashSet, fs};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY06 -------");
    let example = vec![0, 2, 7, 0];
    let input = fs::read_to_string("inputs/input_day06").expect("Unable to read input!");
    let input: Vec<&str> = input.split_whitespace().collect();
    let input: Vec<u32> = input.iter().map(|x| x.parse::<u32>().unwrap()).collect();

    day06_part1(&example, &input);
    day06_part2(&example, &input);
}

fn one_loop(banks: &mut [u32], start_bank_position: usize) {
    let nb_blocks = banks[start_bank_position];
    banks[start_bank_position] = 0;
    let mut position = start_bank_position;
    let nb_banks = banks.len();
    for _ in 0..nb_blocks {
        position += 1;
        position %= nb_banks;
        banks[position] += 1;
    }
}

fn count_redistribution_cycles(banks: &mut [u32]) -> u32 {
    let mut seen_states = HashSet::new();
    seen_states.insert(banks.to_owned());
    let mut nb_cycles = 0;
    loop {
        nb_cycles += 1;
        let mut pos_max = 0;
        let mut max = 0;
        for (i, &val) in banks.iter().enumerate() {
            if val > max {
                pos_max = i;
                max = val;
            }
        }
        one_loop(banks, pos_max);
        let banks_clone = banks.to_owned();
        if seen_states.contains(&banks_clone) {
            return nb_cycles;
        } else {
            seen_states.insert(banks_clone);
        }
    }
}

fn day06_part1(example: &[u32], input: &[u32]) {
    // Exemple tests
    // let mut example_clone1 = example.to_owned();
    // println!("  Start: {example_clone1:?}");
    // one_loop(&mut example_clone1, 2);
    // println!(" 1 loop: {example_clone1:?}");
    // one_loop(&mut example_clone1, 1);
    // println!("2 loops: {example_clone1:?}");
    // one_loop(&mut example_clone1, 0);
    // println!("3 loops: {example_clone1:?}");
    // one_loop(&mut example_clone1, 3);
    // println!("4 loops: {example_clone1:?}");
    // one_loop(&mut example_clone1, 2);
    // println!("4 loops: {example_clone1:?}");
    let mut example_clone2 = example.to_owned();
    assert_eq!(count_redistribution_cycles(&mut example_clone2), 5);

    // Solve puzzle
    let mut input_clone = input.to_owned();
    let res = count_redistribution_cycles(&mut input_clone);
    println!("Result part 1: {res}");
    assert_eq!(res, 12841);
    println!("> DAY06 - part 1: OK!");
}

fn day06_part2(_example: &[u32], _input: &[u32]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY06 - part 2: OK!");
}
