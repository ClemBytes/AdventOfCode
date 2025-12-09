use std::{collections::HashSet, fs};

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
        let (j, i) = line.split_once(",").unwrap();
        red_tiles.push((i.parse().unwrap(), j.parse().unwrap()));
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

fn get_green_and_red_tiles(red_tiles: &[(usize, usize)]) -> HashSet<(usize, usize)> {
    // Initiate by adding all red tiles
    let mut green_and_red_tiles: HashSet<(usize, usize)> = red_tiles.iter().cloned().collect();

    // Then add all green tiles between two red tiles
    let nb_tiles = red_tiles.len();
    for i in 0..nb_tiles {
        let tile1 = red_tiles[i];
        let tile2 = red_tiles[(i + 1) % nb_tiles];
        if tile1.0 == tile2.0 {
            let i = tile1.0;
            let min_j = tile1.1.min(tile2.1);
            let max_j = tile1.1.max(tile2.1);
            for j in min_j + 1..max_j {
                green_and_red_tiles.insert((i, j));
            }
        } else if tile1.1 == tile2.1 {
            let j = tile1.1;
            let min_i = tile1.0.min(tile2.0);
            let max_i = tile1.0.max(tile2.0);
            for i in min_i + 1..max_i {
                green_and_red_tiles.insert((i, j));
            }
        } else {
            unreachable!("'{tile1:?}' and '{tile2:?}' should have one common coordinate!");
        }
    }

    // Now fill, line by line
    let max_i = *green_and_red_tiles.iter().map(|(i, _)| i).max().unwrap();
    let max_j = *green_and_red_tiles.iter().map(|(_, j)| j).max().unwrap();
    for i in 0..max_i {
        let mut inside = false;
        for j in 0..max_j {
            if green_and_red_tiles.contains(&(i, j)) {
                inside = !inside;
            }
            
            if inside {
                green_and_red_tiles.insert((i, j));
            }
        }

    }

    green_and_red_tiles
}

fn day09_part2(example: &[(usize, usize)], _input: &[(usize, usize)]) {
    // Exemple tests
    // Check fill with green
    let raw_example_filled_grid = fs::read_to_string("inputs/example_day09_filled_grid").expect("Unable to read input!");
    let mut example_green_and_red_tiles = HashSet::new();
    for (i, line) in raw_example_filled_grid.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'X' || c == '#' {
                example_green_and_red_tiles.insert((i, j));
            }
        }
    }
    let res_example = get_green_and_red_tiles(example);
    println!("diff e-r: {:?}", example_green_and_red_tiles.difference(&res_example));
    println!("diff r-e: {:?}", res_example.difference(&example_green_and_red_tiles));
    assert_eq!(res_example, example_green_and_red_tiles);
    println!("Examples OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY09 - part 2: OK!");
}
