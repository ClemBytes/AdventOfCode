use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY18 -------");
    let example = fs::read_to_string("inputs/example_day18").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day18").expect("Unable to read input!");
    let input = parse(&input);

    day18_part1(&example, &input);
    day18_part2(&example, &input);
}

const NB_TURNS_EXAMPLE: u32 = 4;
const NB_TURNS_INPUT: u32 = 100;

fn parse(raw_input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![];
    for line in raw_input.lines() {
        let mut new_line: Vec<char> = vec![];
        for ch in line.chars() {
            new_line.push(ch);
        }
        grid.push(new_line);
    }
    grid
}

fn print_grid(grid: &[Vec<char>]) {
    for line in grid {
        for ch in line {
            print!("{ch}");
        }
        println!();
    }
    println!();
}

fn next_step(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = vec![];
    let nb_lines = grid.len() as i32;
    let nb_cols = grid[0].len() as i32;
    for (i, line) in grid.iter().enumerate() {
        let mut new_line: Vec<char> = vec![];
        let i = i as i32;
        for (j, &ch) in line.iter().enumerate() {
            let j = j as i32;
            // Count neighbors ON
            let mut nb_neighbors_on = 0;
            for ni in (i - 1).max(0)..(i + 2).min(nb_cols) {
                'around: for nj in (j - 1).max(0)..(j + 2).min(nb_lines) {
                    if ni == i && nj == j {
                        // Without counting ourselves
                        continue 'around;
                    }
                    if grid[ni as usize][nj as usize] == '#' {
                        nb_neighbors_on += 1;
                    }
                }
            }

            // Update new grid
            if ch == '#' {
                if nb_neighbors_on == 2 || nb_neighbors_on == 3 {
                    new_line.push('#');
                } else {
                    new_line.push('.');
                }
            } else if nb_neighbors_on == 3 {
                new_line.push('#');
            } else {
                new_line.push('.');
            }
        }
        new_grid.push(new_line);
    }
    new_grid
}

fn apply_n_steps(mut grid: Vec<Vec<char>>, n: u32) -> u32 {
    for _ in 0..n {
        grid = next_step(&grid);
    }
    let mut nb_on = 0;
    for line in grid {
        for ch in line {
            if ch == '#' {
                nb_on += 1;
            }
        }
    }
    nb_on
}

fn day18_part1(example: &[Vec<char>], input: &[Vec<char>]) {
    // Exemple tests
    println!("Example initial state:");
    print_grid(example);
    println!("Example after 1 step:");
    print_grid(&next_step(example));
    let res = apply_n_steps(example.to_vec(), NB_TURNS_EXAMPLE);
    println!("Example result part 1: {res}");
    assert_eq!(res, 4);

    // Solve puzzle
    let res = apply_n_steps(input.to_vec(), NB_TURNS_INPUT);
    println!("Result part 1: {res}");
    assert_eq!(res, 1061);
    println!("> DAY18 - part 1: OK!");
}

fn day18_part2(_example: &[Vec<char>], _input: &[Vec<char>]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY18 - part 2: OK!");
}
