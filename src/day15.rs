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

#[derive(Debug, Clone)]
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
        panic!("Total number of teaspoons should be {MAX_TEASPOONS}!\n List: {ingredients_list:?}");
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
    total_capacity.max(0) * total_durability.max(0) * total_flavor.max(0) * total_texture.max(0)
}

fn recursive_search_max(
    ingredients_list_to_empty: &mut VecDeque<(Ingredient, i32)>,
    ingredients_list_to_fill: &mut VecDeque<(Ingredient, i32)>,
) -> i32 {
    if ingredients_list_to_empty.is_empty() {
        let res = compute_score(ingredients_list_to_fill);
        return res;
    }
    let mut max_score = 0;
    let n = ingredients_list_to_empty.len();
    for _ in 0..n {
        let ingredient_tuple = ingredients_list_to_empty.pop_front().unwrap();
        let (ref ingredient, _) = ingredient_tuple;
        let already_used_teaspoons = ingredients_list_to_fill
            .iter()
            .map(|&(_, value)| value)
            .sum::<i32>();
        if n == 1 {
            ingredients_list_to_fill
                .push_back((ingredient.clone(), MAX_TEASPOONS - already_used_teaspoons));
            let score = recursive_search_max(ingredients_list_to_empty, ingredients_list_to_fill);
            if score > max_score {
                max_score = score;
            }
            ingredients_list_to_fill.pop_back();
        } else {
            for tsp in 0..(MAX_TEASPOONS - already_used_teaspoons + 1) {
                ingredients_list_to_fill.push_back((ingredient.clone(), tsp));
                let score =
                    recursive_search_max(ingredients_list_to_empty, ingredients_list_to_fill);
                if score > max_score {
                    max_score = score;
                }
                ingredients_list_to_fill.pop_back();
            }
        }
        ingredients_list_to_empty.push_back(ingredient_tuple);
    }
    max_score
}

fn day15_part1(example: &mut VecDeque<(Ingredient, i32)>, input: &mut VecDeque<(Ingredient, i32)>) {
    // Exemple tests
    assert_eq!(
        recursive_search_max(example, &mut VecDeque::new()),
        62842880
    );

    // Solve puzzle
    let res = recursive_search_max(input, &mut VecDeque::new());
    println!("Result part 1: {res}");
    assert_eq!(res, 13882464);
    println!("> DAY15 - part 1: OK!");
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
