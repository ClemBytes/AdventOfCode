use std::collections::HashMap;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY03 -------");
    let input = 361527;

    day03_part1(input);
    day03_part2(input);
}

fn manhattan_distance(input: u32) -> u32 {
    // Length side nth square in spiral = 2n+1
    // Total length nth square in spiral = 4(2n+1)-4 = 8n
    // Last of each square in spiral is 1 - 9 (1+8×1) - 25 (9+8×2) - 49 (25+8×3)…
    // So last of nth square in spiral is 1 + 8×1 + 8×2 + … + 8×n = 1 + 8×sum(1 to n)
    // So it is 1 + 8 × n(n + 1)/2 = 1 + 4n(n + 1)
    // To know in which square my input is, I need to find the lowest n such that:
    // input <= 1 + 4n(n + 1)
    if input == 1 {
        return 0;
    }
    let mut n = 0;
    while input > 1 + 4 * n * (n + 1) {
        n += 1;
    }
    // println!("{input}: {n}");
    // Now I need to find where is my input in the square
    let len_side_square = 2 * n + 1;
    let last_value_in_previous_square = 2 + 4 * n * (n - 1) - 1;
    // let side = (input - last_value_in_previous_square) / len_side_square;
    let place_in_side = (input - last_value_in_previous_square) % (len_side_square - 1);
    // The middle of the side of the nth square is at place n in the side
    let distance_from_side_middle = place_in_side.max(n) - place_in_side.min(n);
    // println!("input: {input} | n: {n} | last_value_in_previous_square: {last_value_in_previous_square} | len_side_square: {len_side_square} | place_in_side: {place_in_side} | distance_from_side_middle: {distance_from_side_middle}");
    n + distance_from_side_middle
}

fn day03_part1(input: u32) {
    // Exemple tests
    assert_eq!(manhattan_distance(1), 0);
    // println!("manhattan_distance(1) = 0 OK");
    assert_eq!(manhattan_distance(12), 3);
    // println!("manhattan_distance(12) = 3 OK");
    assert_eq!(manhattan_distance(23), 2);
    // println!("manhattan_distance(23) = 2 OK");
    assert_eq!(manhattan_distance(1024), 31);
    // println!("manhattan_distance(1024) = 31 OK");
    println!("Example OK");

    // Solve puzzle
    let res = manhattan_distance(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 326);
    println!("> DAY03 - part 1: OK!");
}

fn solve_part2(input: u32) -> u32 {
    // Just fill the spiral, and keep coordinates
    let mut spiral = HashMap::new();
    let mut value = 1;
    // let mut square = 1;
    let mut x = 0;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 0;
    spiral.insert((x, y), value);
    while value <= input {
        x += dx;
        y += dy;
        let neighbors = [
            spiral.get(&(x - 1, y)),     // down
            spiral.get(&(x - 1, y - 1)), // down-left
            spiral.get(&(x - 1, y + 1)), // down-right
            spiral.get(&(x + 1, y)),     // up
            spiral.get(&(x + 1, y - 1)), // up-left
            spiral.get(&(x + 1, y + 1)), // up-right
            spiral.get(&(x, y - 1)),     // left
            spiral.get(&(x, y + 1)),     // right
        ];
        value = 0;
        for v in neighbors.into_iter().flatten() {
            value += v
        }
        spiral.insert((x, y), value);
        if x > 0 && y == -(x - 1) {
            // right to UP
            dx = 0;
            dy = 1;
        } else if x > 0 && y == x {
            // up to LEFT
            dx = -1;
            dy = 0;
        } else if x < 0 && y == -x {
            // left to DOWN
            dx = 0;
            dy = -1;
        } else if x < 0 && y == x {
            // down to RIGHT
            dx = 1;
            dy = 0;
        }
    }
    value
}

fn day03_part2(input: u32) {
    // Exemple tests
    assert_eq!(solve_part2(1), 2);
    assert_eq!(solve_part2(2), 4);
    assert_eq!(solve_part2(13), 23);
    assert_eq!(solve_part2(25), 26);
    assert_eq!(solve_part2(58), 59);
    assert_eq!(solve_part2(438), 747);

    // Solve puzzle
    let res = solve_part2(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 363010);
    println!("> DAY03 - part 2: OK!");
}
