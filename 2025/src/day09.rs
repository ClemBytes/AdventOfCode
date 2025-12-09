use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY09 -------");
    let example = fs::read_to_string("inputs/example_day09").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day09").expect("Unable to read input!");
    let input = parse(&input);

    day09_part1(&example, &input);
    day09_part2(&example, &input);
}

fn parse(raw_input: &str) -> Vec<(usize, usize)> {
    let mut red_tiles = vec![];
    for line in raw_input.lines() {
        let (x, y) = line.split_once(",").unwrap();
        red_tiles.push((x.parse().unwrap(), y.parse().unwrap()));
    }
    red_tiles
}

fn rectangle_area(tile1: (usize, usize), tile2: (usize, usize)) -> usize {
    (tile1.0.max(tile2.0) - tile1.0.min(tile2.0) + 1)
        * (tile1.1.max(tile2.1) - tile1.1.min(tile2.1) + 1)
}

fn find_largest_rectangle_area(red_tiles: &[(usize, usize)]) -> usize {
    let mut max_area = 0;
    for (i, &tile1) in red_tiles.iter().enumerate() {
        for &tile2 in red_tiles[i + 1..].iter() {
            let area = rectangle_area(tile1, tile2);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

fn day09_part1(example: &[(usize, usize)], input: &[(usize, usize)]) {
    // Exemple tests
    assert_eq!(rectangle_area((2, 5), (9, 7)), 24);
    assert_eq!(rectangle_area((9, 7), (2, 5)), 24);
    assert_eq!(rectangle_area((7, 1), (11, 7)), 35);
    assert_eq!(rectangle_area((11, 7), (7, 1)), 35);
    assert_eq!(rectangle_area((7, 3), (2, 3)), 6);
    assert_eq!(rectangle_area((2, 3), (7, 3)), 6);
    assert_eq!(rectangle_area((2, 5), (11, 1)), 50);
    assert_eq!(rectangle_area((11, 1), (2, 5)), 50);
    assert_eq!(find_largest_rectangle_area(example), 50);

    // Solve puzzle
    let res = find_largest_rectangle_area(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 4744899849);
    println!("> DAY09 - part 1: OK!");
}

fn day09_part2(_example: &[(usize, usize)], _input: &[(usize, usize)]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY09 - part 2: OK!");
}
