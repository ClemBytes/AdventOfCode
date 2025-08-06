use std::{
    collections::{HashMap, HashSet},
    fs,
};

use rand::{rng, seq::SliceRandom};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY19 -------");
    let input = fs::read_to_string("inputs/input_day19").expect("Unable to read input!");
    let input1 = parse_for_part1(&input);
    let input2 = parse_for_part2(&input);

    day19_part1(&input1);
    day19_part2(&input2);
}

fn parse_for_part1(raw_input: &str) -> (HashMap<&str, Vec<&str>>, &str) {
    let mut replacements: HashMap<&str, Vec<&str>> = HashMap::new();
    let (list_of_replacements, molecule) = raw_input.split_once("\n\n").unwrap();
    for line in list_of_replacements.lines() {
        let (before, after) = line.split_once(" => ").unwrap();
        let list_of_replacements = replacements.entry(before).or_default();
        list_of_replacements.push(after);
    }
    (replacements, molecule)
}

fn parse_for_part2(raw_input: &str) -> (Vec<(&str, &str)>, &str) {
    let mut replacements: Vec<(&str, &str)> = vec![];
    let (list_of_replacements, molecule) = raw_input.split_once("\n\n").unwrap();
    for line in list_of_replacements.lines() {
        let (before, after) = line.split_once(" => ").unwrap();
        replacements.push((after, before));
    }
    (replacements, molecule.trim())
}

fn find_nb_distinct_creations(input: &(HashMap<&str, Vec<&str>>, &str)) -> usize {
    let mut creations: HashSet<String> = HashSet::new();
    let (replacements, molecule) = input;
    let n = molecule.len();
    for (i, _) in molecule.char_indices() {
        let before = &molecule[..i].to_string();
        if replacements.contains_key(&molecule[i..i + 1]) {
            let after = &molecule[i + 1..];
            for &repl in replacements.get(&molecule[i..i + 1]).unwrap() {
                let mut new_creation = before.to_owned() + repl;
                new_creation += after;
                creations.insert(new_creation);
            }
        } else if i < n - 1 && replacements.contains_key(&molecule[i..i + 2]) {
            let after = &molecule[i + 2..];
            for &repl in replacements.get(&molecule[i..i + 2]).unwrap() {
                let mut new_creation = before.to_owned() + repl;
                new_creation += after;
                creations.insert(new_creation);
            }
        }
    }
    creations.len()
}

fn day19_part1(input: &(HashMap<&str, Vec<&str>>, &str)) {
    // Exemple tests
    let mut replacements_example: HashMap<&str, Vec<&str>> = HashMap::new();
    replacements_example.insert("H", vec!["HO", "OH"]);
    replacements_example.insert("O", vec!["HH"]);
    let molecule_example = "HOH";
    assert_eq!(
        find_nb_distinct_creations(&(replacements_example.clone(), molecule_example)),
        4
    );
    let molecule_example = "HOHOHO";
    assert_eq!(
        find_nb_distinct_creations(&(replacements_example, molecule_example)),
        7
    );

    // Solve puzzle
    let res = find_nb_distinct_creations(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 576);
    println!("> DAY19 - part 1: OK!");
}

fn nb_steps_to_find_molecule(input: &(Vec<(&str, &str)>, &str)) -> u32 {
    let mut nb_shuffles = 0;
    let mut replacements = input.0.clone();
    let mut molecule = input.1.to_string();
    let mut nb_steps: u32 = 0;
    let old_molecule = molecule.clone();
    'outer: while molecule != "e" {
        for &repl in replacements.iter() {
            let new_molecule = molecule.replacen(repl.0, repl.1, 1);
            if new_molecule != molecule {
                molecule = new_molecule;
                nb_steps += 1;
                continue 'outer;
            }
        }
        // No more replacements possible, so shuffle the replacements list and start again
        let mut rng = rng();
        replacements.shuffle(&mut rng);
        molecule = old_molecule.clone();
        nb_steps = 0;
        nb_shuffles += 1;
    }
    println!("Solution found after {nb_shuffles} shuffle(s)!");
    nb_steps
}

fn day19_part2(input: &(Vec<(&str, &str)>, &str)) {
    // Exemple tests
    let replacements_example: Vec<(&str, &str)> = vec![
        ("HH", "O"),
        ("OH", "H"),
        ("HO", "H"),
        ("O", "e"),
        ("H", "e"),
    ];
    let molecule_example = "HOH";
    assert_eq!(
        nb_steps_to_find_molecule(&(replacements_example.clone(), molecule_example)),
        3
    );
    let molecule_example = "HOHOHO";
    assert_eq!(
        nb_steps_to_find_molecule(&(replacements_example, molecule_example)),
        6
    );

    // Solve puzzle
    let res = nb_steps_to_find_molecule(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 207);
    println!("> DAY19 - part 2: OK!");
}
