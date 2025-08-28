use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY02 -------");
    let example = fs::read_to_string("inputs/example_day02").expect("Unable to read input!");
    let example = Instruction::parse(&example);
    let input = fs::read_to_string("inputs/input_day02").expect("Unable to read input!");
    let input = Instruction::parse(&input);

    day02_part1(&example, &input);
    day02_part2(&example, &input);
}

#[derive(Debug)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl Instruction {
    fn parse(raw_input: &str) -> Vec<Vec<Self>> {
        let mut instructions: Vec<Vec<Instruction>> = vec![];
        for line in raw_input.lines() {
            let mut new_line = vec![];
            for ch in line.chars() {
                let instruction = match ch {
                    'U' => Instruction::Up,
                    'D' => Instruction::Down,
                    'L' => Instruction::Left,
                    'R' => Instruction::Right,
                    other => panic!("Unknown instruction: {other}. Should be 'U', 'D', 'L' or 'R'"),
                };
                new_line.push(instruction);
            }
            instructions.push(new_line);
        }
        instructions
    }

    fn next_number(&self, current_number: u32) -> u32 {
        match current_number {
            1 => match self {
                Instruction::Up => 1,
                Instruction::Down => 4,
                Instruction::Left => 1,
                Instruction::Right => 2,
            },
            2 => match self {
                Instruction::Up => 2,
                Instruction::Down => 5,
                Instruction::Left => 1,
                Instruction::Right => 3,
            },
            3 => match self {
                Instruction::Up => 3,
                Instruction::Down => 6,
                Instruction::Left => 2,
                Instruction::Right => 3,
            },
            4 => match self {
                Instruction::Up => 1,
                Instruction::Down => 7,
                Instruction::Left => 4,
                Instruction::Right => 5,
            },
            5 => match self {
                Instruction::Up => 2,
                Instruction::Down => 8,
                Instruction::Left => 4,
                Instruction::Right => 6,
            },
            6 => match self {
                Instruction::Up => 3,
                Instruction::Down => 9,
                Instruction::Left => 5,
                Instruction::Right => 6,
            },
            7 => match self {
                Instruction::Up => 4,
                Instruction::Down => 7,
                Instruction::Left => 7,
                Instruction::Right => 8,
            },
            8 => match self {
                Instruction::Up => 5,
                Instruction::Down => 8,
                Instruction::Left => 7,
                Instruction::Right => 9,
            },
            9 => match self {
                Instruction::Up => 6,
                Instruction::Down => 9,
                Instruction::Left => 8,
                Instruction::Right => 9,
            },
            other => panic!("Unknown digit: {other}. Should be 1, 2, 3, 4, 5, 6, 7, 8, or 9"),
        }
    }

    fn next_character(&self, current_character: char) -> char {
        match current_character {
            '1' => match self {
                Instruction::Down => '3',
                _ => '1',
            },
            '2' => match self {
                Instruction::Down => '6',
                Instruction::Right => '3',
                _ => '2',
            },
            '3' => match self {
                Instruction::Up => '1',
                Instruction::Down => '7',
                Instruction::Left => '2',
                Instruction::Right => '4',
            },
            '4' => match self {
                Instruction::Down => '8',
                Instruction::Left => '3',
                _ => '4',
            },
            '5' => match self {
                Instruction::Right => '6',
                _ => '5',
            },
            '6' => match self {
                Instruction::Up => '2',
                Instruction::Down => 'A',
                Instruction::Left => '5',
                Instruction::Right => '7',
            },
            '7' => match self {
                Instruction::Up => '3',
                Instruction::Down => 'B',
                Instruction::Left => '6',
                Instruction::Right => '8',
            },
            '8' => match self {
                Instruction::Up => '4',
                Instruction::Down => 'C',
                Instruction::Left => '7',
                Instruction::Right => '9',
            },
            '9' => match self {
                Instruction::Left => '8',
                _ => '9',
            },
            'A' => match self {
                Instruction::Up => '6',
                Instruction::Right => 'B',
                _ => 'A',
            },
            'B' => match self {
                Instruction::Up => '7',
                Instruction::Down => 'D',
                Instruction::Left => 'A',
                Instruction::Right => 'C',
            },
            'C' => match self {
                Instruction::Up => '8',
                Instruction::Left => 'B',
                _ => 'C',
            },
            'D' => match self {
                Instruction::Up => 'B',
                _ => 'D',
            },
            other => panic!(
                "Unknown character: {other}. Should be 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C or D"
            ),
        }
    }
}

fn code_part1(instructions_list: &[Vec<Instruction>], start_digit: u32) -> u32 {
    let mut current_digit = start_digit;
    let mut res = 0;
    let nb_digits = instructions_list.len() as u32;
    for (i, line) in instructions_list.iter().enumerate() {
        for instruction in line {
            current_digit = instruction.next_number(current_digit);
        }
        res += current_digit * 10_u32.pow(nb_digits - 1 - i as u32);
    }
    res
}

fn code_part2(instructions_list: &[Vec<Instruction>], start_character: char) -> String {
    let mut current_character = start_character;
    let mut res = String::new();
    for line in instructions_list {
        for instruction in line {
            current_character = instruction.next_character(current_character);
        }
        res.push(current_character);
    }
    res
}

fn day02_part1(example: &[Vec<Instruction>], input: &[Vec<Instruction>]) {
    // Exemple tests
    assert_eq!(code_part1(example, 5), 1985);

    // Solve puzzle
    let res = code_part1(input, 5);
    println!("Result part 1: {res}");
    assert_eq!(res, 78985);
    println!("> DAY02 - part 1: OK!");
}

fn day02_part2(example: &[Vec<Instruction>], input: &[Vec<Instruction>]) {
    // Exemple tests
    assert_eq!(code_part2(example, '5'), "5DB3");

    // Solve puzzle
    let res = code_part2(input, '5');
    println!("Result part 2: {res}");
    assert_eq!(res, "57DD8");
    println!("> DAY02 - part 2: OK!");
}
