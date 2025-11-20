use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY19 -------");
    let example = fs::read_to_string("inputs/example_day19").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day19").expect("Unable to read input!");
    let input = parse(&input);

    day19_all(&example, &input);
}

fn parse(raw_input: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];
    for line in raw_input.lines() {
        grid.push(line.chars().collect());
    }
    grid
}

fn find_start_position(grid: &[Vec<char>]) -> usize {
    for (i, c) in grid[0].iter().enumerate() {
        if *c == '|' {
            return i;
        }
    }
    unreachable!("Start position not found!");
}

fn find_the_way(grid: &[Vec<char>]) -> (String, i32) {
    let mut letters = String::new();
    let mut nb_steps = 0;
    // Coordinates: (l = line, c = column)
    //    Up: (-1,  0)
    //  Down: (+1,  0)
    //  Left: ( 0, -1)
    // Right: ( 0, +1)
    let (mut l, mut c) = (0usize, find_start_position(grid));
    let mut direction = (1, 0);
    loop {
        let ch = grid[l][c];
        // println!("l: {l} | c: {c} | ch: {ch} | direction: {direction:?}");
        match ch {
            '|' => {
                // Direction doesn't change
                // Just continue
                l = ((l as i32) + direction.0) as usize;
                c = ((c as i32) + direction.1) as usize;
            }
            '-' => {
                // Direction doesn't change
                // Just continue
                l = ((l as i32) + direction.0) as usize;
                c = ((c as i32) + direction.1) as usize;
            }
            '+' => {
                // Change direction, find the new one
                if direction.1 == 0 {
                    // We were going up or down, so look for left or right
                    if grid[l][c - 1] != ' ' {
                        // Left
                        direction = (0, -1);
                    } else if grid[l][c + 1] != ' ' {
                        // Right
                        direction = (0, 1);
                    } else {
                        unreachable!("Not left nor right!")
                    }
                    c = ((c as i32) + direction.1) as usize;
                } else if direction.0 == 0 {
                    // We were going left or right, so look for up or down
                    if grid[l - 1][c] != ' ' {
                        // Up
                        direction = (-1, 0);
                    } else if grid[l + 1][c] != ' ' {
                        // Down
                        direction = (1, 0);
                    } else {
                        unreachable!("Not up nor down!")
                    }
                    l = ((l as i32) + direction.0) as usize;
                }
            }
            ' ' => {
                // Found end
                break;
            }
            _ => {
                // Direction doesn't change
                // Add letter to result and continue
                letters.push(ch);
                l = ((l as i32) + direction.0) as usize;
                c = ((c as i32) + direction.1) as usize;
            }
        }
        nb_steps += 1;
    }
    (letters, nb_steps)
}

fn day19_all(example: &[Vec<char>], input: &[Vec<char>]) {
    // PART 1
    // Exemple tests
    assert_eq!(find_start_position(example), 5);
    let ex = find_the_way(example);
    assert_eq!(ex.0, "ABCDEF");

    // Solve puzzle
    let res = find_the_way(input);
    println!("Result part 1: {}", res.0);
    assert_eq!(res.0, "VTWBPYAQFU");
    println!("> DAY19 - part 1: OK!");

    // PART 2
    // Exemple tests
    assert_eq!(ex.1, 38);

    // Solve puzzle
    println!("Result part 2: {}", res.1);
    assert_eq!(res.1, 17358);
    println!("> DAY19 - part 2: OK!");
}
