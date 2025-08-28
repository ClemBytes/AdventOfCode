use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY18 -------");
    let example1 = fs::read_to_string("inputs/example_day18_1").expect("Unable to read input!");
    let mut example1 = parse(&example1);
    let example2 = fs::read_to_string("inputs/example_day18_2").expect("Unable to read input!");
    let mut example2 = parse(&example2);
    let input = fs::read_to_string("inputs/input_day18").expect("Unable to read input!");
    let mut input = parse(&input);

    day18_part1(&mut example1, &mut input);
    day18_part2(&mut example2, &mut input);
}

const NB_TURNS_EXAMPLE1: u32 = 4;
const NB_TURNS_EXAMPLE2: u32 = 5;
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

fn next_step(grid: &mut [Vec<char>], with_bug: bool) -> Vec<Vec<char>> {
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
    if with_bug {
        let l = nb_lines as usize;
        let c = nb_cols as usize;
        new_grid[0][0] = '#';
        new_grid[l - 1][0] = '#';
        new_grid[0][c - 1] = '#';
        new_grid[l - 1][c - 1] = '#';
    }
    new_grid
}

fn apply_n_steps(mut grid: Vec<Vec<char>>, n: u32, with_bug: bool) -> u32 {
    for _ in 0..n {
        grid = next_step(&mut grid, with_bug);
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

fn day18_part1(example: &mut [Vec<char>], input: &mut [Vec<char>]) {
    // Exemple tests
    println!("Example 1 initial state:");
    print_grid(example);
    println!("Example 1 after 1 step:");
    print_grid(&next_step(example, false));
    let res = apply_n_steps(example.to_vec(), NB_TURNS_EXAMPLE1, false);
    assert_eq!(res, 4);

    // Solve puzzle
    let res = apply_n_steps(input.to_vec(), NB_TURNS_INPUT, false);
    println!("Result part 1: {res}");
    assert_eq!(res, 1061);
    println!("> DAY18 - part 1: OK!");
}

fn day18_part2(example: &mut [Vec<char>], input: &mut [Vec<char>]) {
    // Exemple tests
    println!("Example 2 initial state:");
    print_grid(example);
    println!("Example 2 after 1 step:");
    print_grid(&next_step(example, true));
    let res = apply_n_steps(example.to_vec(), NB_TURNS_EXAMPLE2, true);
    assert_eq!(res, 17);

    // Solve puzzle
    let res = apply_n_steps(input.to_vec(), NB_TURNS_INPUT, true);
    println!("Result part 1: {res}");
    assert_eq!(res, 1006);
    println!("> DAY18 - part 2: OK!");
}
