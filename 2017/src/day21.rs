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
    day21_part2(&input, &start);
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

fn pattern_to_nb(pattern: &[Vec<usize>]) -> (usize, usize) {
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
    binary_nb_vec_reverse
        .chunks_exact(size)
        .map(|c| c.to_vec())
        .collect()
}

fn rotate_pattern(pattern: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let size = pattern.len();
    let mut output = vec![];

    // First transpose
    for i in 0..size {
        let mut l = vec![];
        for pj in pattern.iter().take(size) {
            l.push(pj[i]);
        }
        output.push(l);
    }

    // Then reverse each line
    for oi in output.iter_mut().take(size) {
        oi.reverse();
    }

    output
}

fn flip_pattern(pattern: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut output = pattern.to_vec();
    for oi in output.iter_mut().take(pattern.len()) {
        oi.reverse();
    }
    output
}

fn iterate(
    grid: &[Vec<usize>],
    rules: &HashMap<(usize, usize), (usize, usize)>,
) -> Vec<Vec<usize>> {
    let size_grid_in = grid.len();
    let size_chunk;
    if size_grid_in % 2 == 0 {
        size_chunk = 2;
    } else if size_grid_in % 3 == 0 {
        size_chunk = 3;
    } else {
        unreachable!("size_grid_in '{size_grid_in}' should be divisible by 2 or 3!");
    }
    let nb_chunks = size_grid_in / size_chunk;

    // Split grid into cases => list of patterns
    let mut patterns = vec![];
    for line_chunck in 0..nb_chunks {
        for column_chunck in 0..nb_chunks {
            let mut pattern = vec![];
            for row in &grid[line_chunck * size_chunk..(line_chunck + 1) * size_chunk] {
                pattern.push(
                    row[column_chunck * size_chunk..(column_chunck + 1) * size_chunk].to_vec(),
                );
            }
            patterns.push(pattern);
        }
    }

    // Convert each pattern into a number, find corresponding rule. If no, rotate and flip as necessary.
    let mut patterns_converted_nb = vec![];
    'main_loop: for pattern in patterns {
        let n = pattern_to_nb(&pattern);
        if rules.contains_key(&n) {
            patterns_converted_nb.push(rules.get(&n).unwrap());
        } else {
            let mut rotated_pattern = pattern.to_vec();
            for _ in 0..3 {
                rotated_pattern = rotate_pattern(&rotated_pattern);
                let n = pattern_to_nb(&rotated_pattern);
                if rules.contains_key(&n) {
                    patterns_converted_nb.push(rules.get(&n).unwrap());
                    continue 'main_loop;
                }
            }

            let flipped_pattern = flip_pattern(&pattern);
            let n = pattern_to_nb(&flipped_pattern);
            if rules.contains_key(&n) {
                patterns_converted_nb.push(rules.get(&n).unwrap());
                continue 'main_loop;
            }

            let mut rotated_flipped_pattern = flipped_pattern.clone();
            for _ in 0..3 {
                rotated_flipped_pattern = rotate_pattern(&rotated_flipped_pattern);
                let n = pattern_to_nb(&rotated_flipped_pattern);
                if rules.contains_key(&n) {
                    patterns_converted_nb.push(rules.get(&n).unwrap());
                    continue 'main_loop;
                }
            }

            unreachable!("No rule found for the following pattern:\n{pattern:#?}");
        }
    }

    // Now reconstitute next grid
    let mut next_grid = vec![];
    let chunk_size = patterns_converted_nb[0].0;
    for chunk_row in 0..nb_chunks {
        let start = chunk_row * nb_chunks;
        let end = start + nb_chunks;
        let line_patterns: Vec<Vec<Vec<usize>>> = patterns_converted_nb[start..end]
            .iter()
            .map(|&&n| nb_to_pattern(n))
            .collect();

        for i in 0..chunk_size {
            let mut new_line = vec![];
            for p in &line_patterns {
                new_line.extend(&p[i])
            }
            next_grid.push(new_line);
        }
    }

    next_grid
}

fn solve(
    n: usize,
    rules: &HashMap<(usize, usize), (usize, usize)>,
    start: &[[usize; 3]; 3],
) -> usize {
    let mut grid = vec![];
    for line in start {
        grid.push(line.to_vec());
    }

    // Iterate n times
    for _ in 0..n {
        grid = iterate(&grid, rules);
    }

    // Count nb in 1s
    grid.iter().map(|row| row.iter().sum::<usize>()).sum()
}

fn day21_part1(
    example: &HashMap<(usize, usize), (usize, usize)>,
    input: &HashMap<(usize, usize), (usize, usize)>,
    start: &[[usize; 3]; 3],
) {
    // Exemple tests
    assert_eq!(solve(2, example, start), 12);

    // Solve puzzle
    let res = solve(5, input, start);
    println!("Result part 1: {res}");
    assert_eq!(res, 142);
    println!("> DAY21 - part 1: OK!");
}

fn day21_part2(input: &HashMap<(usize, usize), (usize, usize)>, start: &[[usize; 3]; 3]) {
    // Solve puzzle
    let res = solve(18, input, start);
    println!("Result part 2: {res}");
    assert_eq!(res, 1879071);
    println!("> DAY21 - part 2: OK!");
}
