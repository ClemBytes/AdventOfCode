use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY04 -------");
    let example = fs::read_to_string("inputs/example_day04").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day04").expect("Unable to read input!");
    let input = parse(&input);

    day04_part1(&example, &input);
    day04_part2(&example, &input);
}

fn parse(raw_input: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];
    for line in raw_input.lines() {
        grid.push(line.chars().collect());
    }
    grid
}

fn get_adjacent_coordinates(
    i: usize,
    j: usize,
    nb_rows: usize,
    nb_cols: usize,
) -> Vec<(usize, usize)> {
    let mut adjacent_coordinates = vec![];
    if i > 0 {
        // left
        adjacent_coordinates.push((i - 1, j));
        if j > 0 {
            // up-left
            adjacent_coordinates.push((i - 1, j - 1));
        }
        if j < nb_cols - 1 {
            // down-left
            adjacent_coordinates.push((i - 1, j + 1));
        }
    }
    if i < nb_rows - 1 {
        // right
        adjacent_coordinates.push((i + 1, j));
        if j > 0 {
            // up-right
            adjacent_coordinates.push((i + 1, j - 1));
        }
        if j < nb_cols - 1 {
            // down-right
            adjacent_coordinates.push((i + 1, j + 1));
        }
    }
    if j > 0 {
        // up
        adjacent_coordinates.push((i, j - 1));
    }
    if j < nb_cols - 1 {
        // down
        adjacent_coordinates.push((i, j + 1));
    }
    adjacent_coordinates
}

fn count_accessible_rolls(grid: &[Vec<char>]) -> usize {
    let mut res = 0;
    let nb_rows = grid.len();
    let nb_cols = grid[0].len();
    for (i, line) in grid.iter().enumerate() {
        for (j, &place) in line.iter().enumerate() {
            if place == '@' {
                let mut nb_adjacent_rolls = 0;
                for pos in get_adjacent_coordinates(i, j, nb_rows, nb_cols) {
                    if grid[pos.0][pos.1] == '@' {
                        nb_adjacent_rolls += 1;
                    }
                }
                if nb_adjacent_rolls < 4 {
                    res += 1;
                }
            }
        }
    }
    res
}

fn day04_part1(example: &[Vec<char>], input: &[Vec<char>]) {
    // Exemple tests
    assert_eq!(count_accessible_rolls(example), 13);

    // Solve puzzle
    let res = count_accessible_rolls(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 1393);
    println!("> DAY04 - part 1: OK!");
}

fn day04_part2(_example: &[Vec<char>], _input: &[Vec<char>]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY04 - part 2: OK!");
}
