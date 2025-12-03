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

fn parse(raw_input: &str) -> Vec<Vec<u64>> {
    let mut banks: Vec<_> = vec![];
    for line in raw_input.lines() {
        let mut bank = vec![];
        for digit in line.chars() {
            bank.push(digit.to_digit(10).unwrap() as u64);
        }
        banks.push(bank);
    }
    banks
}

fn max_joltage(n: usize, bank: &[u64]) -> u64 {
    let nb_batteries = bank.len();
    let mut index = 0;
    let mut max_battery = 0;
    for k in 1..n + 1 {
        let mut mb = 0;
        let to_skip = match k {
            1 => index,
            _ => index + 1,
        };
        for (i, &bank_i) in bank
            .iter()
            .enumerate()
            .take(nb_batteries - (n - k))
            .skip(to_skip)
        {
            if bank_i > mb {
                index = i;
                mb = bank_i;
            }
            if mb == 9 {
                break;
            }
        }
        max_battery += mb * 10_u64.pow(n as u32 - k as u32);
    }
    max_battery
}

fn output_joltage(nb_batteries: usize, banks: &[Vec<u64>]) -> u64 {
    let mut sum = 0;
    for bank in banks {
        sum += max_joltage(nb_batteries, bank);
    }
    sum
}

fn day03_part1(example: &[Vec<u64>], input: &[Vec<u64>]) {
    // Exemple tests
    assert_eq!(output_joltage(2, example), 357);

    // Solve puzzle
    let res = output_joltage(2, input);
    println!("Result part 1: {res}");
    assert_eq!(res, 17613);
    println!("> DAY03 - part 1: OK!");
}

fn day03_part2(example: &[Vec<u64>], input: &[Vec<u64>]) {
    // Exemple tests
    let ex: Vec<u64> = "987654321111111"
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect();
    assert_eq!(max_joltage(12, &ex), 987654321111);
    let ex: Vec<u64> = "811111111111119"
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect();
    assert_eq!(max_joltage(12, &ex), 811111111119);
    let ex: Vec<u64> = "234234234234278"
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect();
    assert_eq!(max_joltage(12, &ex), 434234234278);
    let ex: Vec<u64> = "818181911112111"
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect();
    assert_eq!(max_joltage(12, &ex), 888911112111);
    assert_eq!(output_joltage(12, example), 3121910778619);

    // Solve puzzle
    let res = output_joltage(12, input);
    println!("Result part 2: {res}");
    assert_eq!(res, 175304218462560);
    println!("> DAY03 - part 2: OK!");
}
