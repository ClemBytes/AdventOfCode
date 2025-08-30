use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY18 -------");
    let example1 = "..^^.";
    let example1 = Tile::from_str(example1);
    let example2 = ".^^.^.^^^^";
    let example2 = Tile::from_str(example2);
    let input = fs::read_to_string("inputs/input_day18").expect("Unable to read input!");
    let input = Tile::from_str(&input);

    day18_part1(&example1, &example2, &input);
    day18_part2(&input);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Safe,
    Trap,
}

impl Tile {
    fn from_str(raw_input: &str) -> Vec<Self> {
        let mut tiles = vec![];
        for ch in raw_input.trim().chars() {
            let new_tile = match ch {
                '.' => Tile::Safe,
                '^' => Tile::Trap,
                _ => panic!("Unknown tile: {ch}, should be '.' or '^'"),
            };
            tiles.push(new_tile);
        }
        tiles
    }

    fn next_row(row: &[Self]) -> Vec<Self> {
        let mut next_row = Vec::new();
        let nb_tiles = row.len();
        for i in 0..nb_tiles {
            let mut left = Tile::Safe;
            let center = row[i];
            let mut right = Tile::Safe;
            if i != 0 {
                left = row[i - 1];
            }
            if i != nb_tiles - 1 {
                right = row[i + 1];
            }

            let new_tile = match (left, center, right) {
                (Tile::Trap, Tile::Trap, Tile::Safe) => Tile::Trap,
                (Tile::Safe, Tile::Trap, Tile::Trap) => Tile::Trap,
                (Tile::Trap, Tile::Safe, Tile::Safe) => Tile::Trap,
                (Tile::Safe, Tile::Safe, Tile::Trap) => Tile::Trap,
                _ => Tile::Safe,
            };
            next_row.push(new_tile);
        }
        next_row
    }
}

fn nb_safe(first_row: &[Tile], nb_rows: usize) -> usize {
    let mut row = first_row.to_owned();
    let mut nb_safe = 0;
    for _ in 0..nb_rows {
        nb_safe += row
            .clone()
            .into_iter()
            .filter(|tile| matches!(tile, Tile::Safe))
            .count();
        row = Tile::next_row(&row);
    }
    nb_safe
}

fn day18_part1(example1: &[Tile], example2: &[Tile], input: &[Tile]) {
    // Exemple tests
    assert_eq!(
        Tile::next_row(example1),
        vec![Tile::Safe, Tile::Trap, Tile::Trap, Tile::Trap, Tile::Trap]
    );
    assert_eq!(
        Tile::next_row(&[Tile::Safe, Tile::Trap, Tile::Trap, Tile::Trap, Tile::Trap]),
        vec![Tile::Trap, Tile::Trap, Tile::Safe, Tile::Safe, Tile::Trap]
    );
    assert_eq!(nb_safe(example1, 3), 6);
    assert_eq!(nb_safe(example2, 10), 38);

    // Solve puzzle
    let res = nb_safe(input, 40);
    println!("Result part 1: {res}");
    assert_eq!(res, 1961);
    println!("> DAY18 - part 1: OK!");
}

fn day18_part2(input: &[Tile]) {
    // Solve puzzle
    let res = nb_safe(input, 400000);
    println!("Result part 2: {res}");
    assert_eq!(res, 20000795);
    println!("> DAY18 - part 2: OK!");
}
