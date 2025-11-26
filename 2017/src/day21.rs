use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY21 -------");
    let example = fs::read_to_string("inputs/example_day21").expect("Unable to read input!");
    let example = Rule::parse(&example);
    let input = fs::read_to_string("inputs/input_day21").expect("Unable to read input!");
    let input = Rule::parse(&input);

    day21_part1(&example, &input);
    day21_part2(&example, &input);
}

#[derive(Debug, Clone, Copy)]
enum State {
    On,
    Off,
}

#[derive(Debug, Clone)]
struct Rule {
    size_in: usize,
    input_pattern: Vec<Vec<State>>,
    output_pattern: Vec<Vec<State>>,
}

impl Rule {
    fn size_out(&self) -> usize {
        match self.size_in {
            2 => 3,
            3 => 4,
            other => unreachable!("Impossible size_in for Rule: {other}! Should be 2 or 3!"),
        }
    }

    fn parse(raw_input: &str) -> Vec<Rule> {
        let mut rules = vec![];
        for line in raw_input.lines() {
            let line = line.trim();
            let (i, o) = line.split_once(" => ").unwrap();
            let i_lines: Vec<&str> = i.split("/").collect();
            let size_in = i_lines.len();
            if size_in != 2 && size_in != 3 {
                unreachable!("Impossible size_in {size_in} for Rule {line}! Should be 2 or 3!");
            }
            let mut input_pattern = vec![];
            for i_line in i_lines {
                let mut l = vec![];
                for c in i_line.chars() {
                    let s = match c {
                        '#' => State::On,
                        '.' => State::Off,
                        other => unreachable!("Unknown state: '{other}' in line {line}! Should be '#' or '.''!"),
                    };
                    l.push(s);
                }
                input_pattern.push(l);
            }

            let o_lines: Vec<&str> = o.split("/").collect();
            let mut output_pattern = vec![];
            for o_line in o_lines {
                let mut l = vec![];
                for c in o_line.chars() {
                    let s = match c {
                        '#' => State::On,
                        '.' => State::Off,
                        other => unreachable!("Unknown state: '{other}' in line {line}! Should be '#' or '.''!"),
                    };
                    l.push(s);
                }
                output_pattern.push(l);
            }

            rules.push(Rule {size_in, input_pattern, output_pattern});
        }
        rules
    }
}

// Idea: represent patterns with powers of 2:
// .#/#. = 0×2⁰ + 1×2¹ + 1×2² + 0×2³ = 0 + 2 + 4 + 0 = 6 = 0110b
// #./.# = 1×2⁰ + 0×2¹ + 0×2² + 1×2³ = 1 + 0 + 0 + 8 = 9 = 1001b
// ##/## = 1×2⁰ + 1×2¹ + 1×2² + 1×2³ = 1 + 2 + 4 + 8 = 15 = 1111b
// Well, it's just a binary representation

fn day21_part1(_example: &Vec<Rule>, _input: &Vec<Rule>) {
    let a = 9;
    println!("{a:b}");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY21 - part 1: OK!");
}

fn day21_part2(_example: &Vec<Rule>, _input: &Vec<Rule>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY21 - part 2: OK!");
}
