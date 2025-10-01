use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY02 -------");
    let example1 = fs::read_to_string("inputs/example_day02_part1").expect("Unable to read input!");
    let example1 = parse(&example1);
    let example2 = fs::read_to_string("inputs/example_day02_part2").expect("Unable to read input!");
    let example2 = parse(&example2);
    let input = fs::read_to_string("inputs/input_day02").expect("Unable to read input!");
    let input = parse(&input);

    day02_part1(&example1, &input);
    day02_part2(&example2, &input);
}

fn parse(raw_input: &str) -> Vec<Vec<u32>> {
    let mut spreadsheet: Vec<Vec<u32>> = vec![];
    for line in raw_input.lines() {
        let l: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        spreadsheet.push(l);
    }
    spreadsheet
}

fn checksum(spreadsheet: &[Vec<u32>]) -> u32 {
    let mut s = 0;
    for line in spreadsheet {
        s += line.iter().max().unwrap() - line.iter().min().unwrap();
    }
    s
}

fn day02_part1(example: &[Vec<u32>], input: &[Vec<u32>]) {
    // Exemple tests
    assert_eq!(checksum(example), 18);
    println!("Example OK");

    // Solve puzzle
    let res = checksum(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 39126);
    println!("> DAY02 - part 1: OK!");
}

fn sum_divisible(spreadsheet: &[Vec<u32>]) -> u32 {
    let mut s = 0;
    for line in spreadsheet {
        let len_line = line.len();
        'outer: for (i, &n) in line.iter().enumerate() {
            for &m in line.iter().take(len_line).skip(i + 1) {
                let big = n.max(m);
                let small = n.min(m);
                if big % small == 0 {
                    s += big / small;
                    break 'outer;
                }
            }
        }
    }
    s
}

fn day02_part2(example: &[Vec<u32>], input: &[Vec<u32>]) {
    // Exemple tests
    assert_eq!(sum_divisible(example), 9);
    println!("Example OK");

    // Solve puzzle
    let res = sum_divisible(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 258);
    println!("> DAY02 - part 2: OK!");
}
