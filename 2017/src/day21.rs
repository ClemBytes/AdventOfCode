use std::{collections::HashMap, fs};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY21 -------");
    let example = fs::read_to_string("inputs/example_day21").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day21").expect("Unable to read input!");
    let input = parse(&input);

    let start = [[0, 1, 0], [0, 0, 1], [1, 1, 1]];

    day21_part1(&example, &input, &start);
    day21_part2(&example, &input, &start);
}

// Idea: represent patterns with powers of 2:
// .#/#. = 0×2⁰ + 1×2¹ + 1×2² + 0×2³ = 0 + 2 + 4 + 0 = 6 = 0110b
// #./.# = 1×2⁰ + 0×2¹ + 0×2² + 1×2³ = 1 + 0 + 0 + 8 = 9 = 1001b
// ##/## = 1×2⁰ + 1×2¹ + 1×2² + 1×2³ = 1 + 2 + 4 + 8 = 15 = 1111b
// Well, it's just a binary representation
// The example rules become:
// ../.# => ##./#../...                 : 8 => 11
// .#./..#/### => #..#/..../..../#..#   : 482 => 36_873
// And if we add sizes:
// ../.# => ##./#../...                 : (2, 8) => (3, 11)
// .#./..#/### => #..#/..../..../#..#   : (3, 482) => (4, 36_873)

fn parse(raw_input: &str) -> HashMap<(usize, usize), (usize, usize)> {
    let mut rules = HashMap::new();
    for line in raw_input.lines() {
        let (i, o) = line.split_once(" => ").unwrap();
        let mut input_pattern = 0;
        let mut counter = 0;
        for c in i.chars() {
            match c {
                '#' => {
                    input_pattern += 2_usize.pow(counter);
                    counter += 1;
                }
                '.' => {
                    counter += 1;
                }
                '/' => {}
                other => {
                    unreachable!("Unknown state: '{other}' in line {line}! Should be '#' or '.''!")
                }
            };
        }
        let size_in = match counter {
            4 => 2,
            9 => 3,
            other => unreachable!("Impossible size_in {other} for Rule {line}! Should be 4 or 9!"),
        };

        let mut output_pattern = 0;
        let mut counter = 0;
        for c in o.chars() {
            match c {
                '#' => {
                    output_pattern += 2_usize.pow(counter);
                    counter += 1;
                }
                '.' => {
                    counter += 1;
                }
                '/' => {}
                other => {
                    unreachable!("Unknown state: '{other}' in line {line}! Should be '#' or '.''!")
                }
            };
        }
        let size_out = match counter {
            9 => 3,
            16 => 4,
            other => {
                unreachable!("Impossible size_out {other} for Rule {line}! Should be 9 or 16!")
            }
        };

        rules.insert((size_in, input_pattern), (size_out, output_pattern));
    }
    rules
}

fn pattern_to_nb(pattern: &Vec<Vec<usize>>) -> (usize, usize) {
    let size = pattern.len();
    let mut res = 0;
    let mut counter = 0;
    for line in pattern {
        for &v in line {
            match v {
                0 => {}
                1 => res += 2_usize.pow(counter),
                other => unreachable!("Should be 0 or 1, not {other}!"),
            }
            counter += 1;
        }
    }
    (size, res)
}

fn nb_to_pattern(input: (usize, usize)) -> Vec<Vec<usize>> {
    let (size, nb) = input;
    let width = size.pow(2);
    let binary_nb_string = format!("{nb:0width$b}");
    let mut binary_nb_vec_reverse: Vec<usize> = binary_nb_string
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();
    binary_nb_vec_reverse.reverse();
    let pattern = binary_nb_vec_reverse
        .chunks_exact(size)
        .map(|c| c.to_vec())
        .collect();
    pattern
}

fn day21_part1(
    _example: &HashMap<(usize, usize), (usize, usize)>,
    input: &HashMap<(usize, usize), (usize, usize)>,
    _start: &[[i32; 3]; 3],
) {
    println!("{input:?}");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY21 - part 1: OK!");
}

fn day21_part2(
    _example: &HashMap<(usize, usize), (usize, usize)>,
    _input: &HashMap<(usize, usize), (usize, usize)>,
    _start: &[[i32; 3]; 3],
) {
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
