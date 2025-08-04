use std::{collections::VecDeque, fs};

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY15 -------");
    let example = fs::read_to_string("inputs/example_day15").expect("Unable to read input!");
    let mut example = parse(&example);
    let input = fs::read_to_string("inputs/input_day15").expect("Unable to read input!");
    let mut input = parse(&input);

    day15_part1(&mut example, &mut input);
    day15_part2(&mut example, &mut input);
}

const MAX_TEASPOONS: i32 = 100;

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    // calories: i32,
}

fn parse(raw_input: &str) -> VecDeque<(Ingredient, i32)> {
    let mut ingredients_list: VecDeque<(Ingredient, i32)> = VecDeque::new();
    let re = Regex::new(r"^[A-Za-z]+: capacity (-?[0-9]+), durability (-?[0-9]+), flavor (-?[0-9]+), texture (-?[0-9]+), calories ([0-9]+)$").unwrap();
    for line in raw_input.lines() {
        let matches = re.captures(line).unwrap();
        let capacity: i32 = matches[1].parse().unwrap();
        let durability: i32 = matches[2].parse().unwrap();
        let flavor: i32 = matches[3].parse().unwrap();
        let texture: i32 = matches[4].parse().unwrap();
        // let calories: i32 = matches[5].parse().unwrap();
        let new_ingredient = Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            // calories,
        };
        ingredients_list.push_back((new_ingredient, 0));
    }
    ingredients_list
}

fn compute_score(ingredients_list: &VecDeque<(Ingredient, i32)>) -> i32 {
    if ingredients_list
        .iter()
        .map(|&(_, value)| value)
        .sum::<i32>()
        != MAX_TEASPOONS
    {
        panic!("Total number of teaspoons should be {MAX_TEASPOONS}!");
    }
    let mut total_capacity: i32 = 0;
    let mut total_durability: i32 = 0;
    let mut total_flavor: i32 = 0;
    let mut total_texture: i32 = 0;
    for (ingredient, nb_teaspoons) in ingredients_list {
        total_capacity += nb_teaspoons * ingredient.capacity;
        total_durability += nb_teaspoons * ingredient.durability;
        total_flavor += nb_teaspoons * ingredient.flavor;
        total_texture += nb_teaspoons * ingredient.texture;
    }
    if total_capacity < 0 {
        total_capacity = 0
    };
    if total_durability < 0 {
        total_durability = 0
    };
    if total_flavor < 0 {
        total_flavor = 0
    };
    if total_texture < 0 {
        total_texture = 0
    };
    total_capacity * total_durability * total_flavor * total_texture
}

// recursive_search_max(
//     ingredients_list: &mut VecDeque<(Ingredient, i32)>>,
// ) -> i32 {

// }

fn day15_part1(
    example: &mut VecDeque<(Ingredient, i32)>,
    _input: &mut VecDeque<(Ingredient, i32)>,
) {
    // Exemple tests
    example[0].1 = 44;
    example[1].1 = 56;
    assert_eq!(compute_score(example), 62842880);

    // Solve puzzle
    // println!("Result part 1: {}");
    // assert_eq!(, );
    // println!("> DAY15 - part 1: OK!");
}

fn day15_part2(
    _example: &mut VecDeque<(Ingredient, i32)>,
    _input: &mut VecDeque<(Ingredient, i32)>,
) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // println!("Result part 2: {}");
    // assert_eq!(, );
    // println!("> DAY15 - part 2: OK!");
}
