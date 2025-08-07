use std::fs;

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

    day21_part1(&example, &input);
    day21_part2(&example, &input);
}

struct Person {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

enum ItemType {
    Weapon,
    Armor,
    Ring,
}

struct Item {
    item_type: ItemType,
    cost: i32,
    damage: i32,
    armor: i32,
}

fn parse(raw_input: &String) -> Vec<_> {
    let mut yyy: Vec<_> = vec![];
    for _line in raw_input.lines() {
        // TODO
        // yyy.push(line);
    }
    yyy
}

fn day21_part1(_example: &Vec<_>, _input: &Vec<_>) {
    println!("TODO - part1");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res = 
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY21 - part 1: OK!");
}

fn day21_part2(_example: &Vec<_>, _input: &Vec<_>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res = 
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY21 - part 2: OK!");
}
