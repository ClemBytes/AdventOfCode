use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY06 -------");
    let example = fs::read_to_string("inputs/example_day06").expect("Unable to read input!");
    let input = fs::read_to_string("inputs/input_day06").expect("Unable to read input!");

    day06_part1(example.clone(), input.clone());
    day06_part2(example, input);
}

fn solve_part1(raw_input: String) -> usize {
    // Firt parse raw_input
    let mut raw_lines = vec![];
    for line in raw_input.lines() {
        let raw_line_splitted: Vec<&str> = line.split_whitespace().collect();
        raw_lines.push(raw_line_splitted);
    }

    // Then parse numbers
    let nb_lines = raw_lines.len();
    let operations = raw_lines[nb_lines - 1].clone();
    let mut parsed_numbers = vec![];
    for raw_lines_i in raw_lines.iter().take(nb_lines - 1) {
        let mut parsed_line = vec![];
        for nb in raw_lines_i.clone() {
            parsed_line.push(nb.parse::<usize>().unwrap());
        }
        parsed_numbers.push(parsed_line);
    }

    // Now find grand total
    let mut res = 0;
    for (i, &op) in operations.iter().enumerate() {
        let mut partial_res = 0;
        match op {
            "+" => {
                for parsed_numbers_l in parsed_numbers.iter().take(nb_lines - 1) {
                    partial_res += parsed_numbers_l[i];
                }
            }
            "*" => {
                partial_res = 1;
                for parsed_numbers_l in parsed_numbers.iter().take(nb_lines - 1) {
                    partial_res *= parsed_numbers_l[i];
                }
            }
            _ => unreachable!("Unknown operation '{op}', should be '+' or '*'!"),
        }
        res += partial_res;
    }
    res
}

fn day06_part1(raw_example: String, raw_input: String) {
    // Exemple tests
    assert_eq!(solve_part1(raw_example), 4277556);

    // Solve puzzle
    let res = solve_part1(raw_input);
    println!("Result part 1: {res}");
    assert_eq!(res, 4387670995909);
    println!("> DAY06 - part 1: OK!");
}

fn solve_part2(raw_input: String) -> usize {
    // Firt parse raw_input as a grid
    let mut grid_input: Vec<Vec<char>> = vec![];
    for line in raw_input.lines() {
        grid_input.push(line.chars().collect());
    }

    // Then directly read and compute
    let nb_lines = grid_input.len();
    let last_line_index = nb_lines - 1;
    let nb_cols = grid_input[0].len();
    let mut current_op = grid_input[last_line_index][0];
    let mut res = 0;
    let mut current_numbers = vec![];
    for col in 0..nb_cols {
        // Check if empty column
        let mut empty_col = true;
        for grid_input_l in grid_input.iter().take(nb_lines) {
            if grid_input_l[col] != ' ' {
                empty_col = false;
                break;
            }
        }
        // If all empty, compute
        if empty_col {
            let mut partial_res = 0;
            match current_op {
                '+' => {
                    for nb in &current_numbers {
                        partial_res += nb;
                    }
                }
                '*' => {
                    partial_res = 1;
                    for nb in &current_numbers {
                        partial_res *= nb;
                    }
                }
                _ => unreachable!("Unknown operation '{current_op}', should be '+' or '*'!"),
            }
            current_numbers.clear();
            res += partial_res;
            current_op = grid_input[last_line_index][col + 1];
            continue;
        }

        // Else, get current number
        let mut current_number = String::new();
        for grid_input_l in grid_input.iter().take(last_line_index) {
            let c = grid_input_l[col];
            if c != ' ' {
                current_number.push(c);
            }
        }
        if !current_number.is_empty() {
            current_numbers.push(current_number.parse::<usize>().unwrap());
        }
    }
    // Final compute
    let mut partial_res = 0;
    match current_op {
        '+' => {
            for nb in current_numbers {
                partial_res += nb;
            }
        }
        '*' => {
            partial_res = 1;
            for nb in current_numbers {
                partial_res *= nb;
            }
        }
        _ => unreachable!("Unknown operation '{current_op}', should be '+' or '*'!"),
    }
    res += partial_res;

    res
}

fn day06_part2(example: String, input: String) {
    // Exemple tests
    assert_eq!(solve_part2(example), 3263827);

    // Solve puzzle
    let res = solve_part2(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 9625320374409);
    println!("> DAY06 - part 2: OK!");
}
