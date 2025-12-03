use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY03 -------");
    let example = fs::read_to_string("inputs/example_day03").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day03").expect("Unable to read input!");
    let input = parse(&input);

    day03_part1(&example, &input);
    day03_part2(&example, &input);
}

fn parse(raw_input: &str) -> Vec<Vec<u32>> {
    let mut banks: Vec<_> = vec![];
    for line in raw_input.lines() {
        let mut bank = vec![];
        for digit in line.chars() {
            bank.push(digit.to_digit(10).unwrap());
        }
        banks.push(bank);
    }
    banks
}

fn max_joltage(bank: &[u32]) -> u32 {
    let nb_batteries = bank.len();

    // Find first max of bank, not being the last one
    let mut index = 0;
    let mut max_battery = 0;
    for (i, &bank_i) in bank.iter().enumerate().take(nb_batteries - 1) {
        if bank_i > max_battery {
            index = i;
            max_battery = bank_i;
        }
        if max_battery == 9 {
            break;
        }
    }

    // Then find second max battery, after the one found
    let mut second_max_battery = 0;
    for &bank_i in bank.iter().take(nb_batteries).skip(index + 1) {
        if bank_i > second_max_battery {
            second_max_battery = bank_i;
        }
        if second_max_battery == 9 {
            break;
        }
    }

    max_battery * 10 + second_max_battery
}

fn output_joltage(banks: &[Vec<u32>]) -> u32 {
    let mut sum = 0;
    for bank in banks {
        sum += max_joltage(bank);
    }
    sum
}

fn day03_part1(example: &[Vec<u32>], input: &[Vec<u32>]) {
    // Exemple tests
    assert_eq!(output_joltage(example), 357);

    // Solve puzzle
    let res = output_joltage(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 17613);
    println!("> DAY03 - part 1: OK!");
}

fn day03_part2(_example: &[Vec<u32>], _input: &[Vec<u32>]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY03 - part 2: OK!");
}
