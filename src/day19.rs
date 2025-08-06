use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY19 -------");
    let input = fs::read_to_string("inputs/input_day19").expect("Unable to read input!");
    let input = parse(&input);

    day19_part1(&input);
    day19_part2(&input);
}

fn parse(raw_input: &str) -> (HashMap<&str, Vec<&str>>, &str) {
    let mut replacements: HashMap<&str, Vec<&str>> = HashMap::new();
    let (list_of_replacements, molecule) = raw_input.split_once("\n\n").unwrap();
    for line in list_of_replacements.lines() {
        let (before, after) = line.split_once(" => ").unwrap();
        let list_of_replacements = replacements.entry(before).or_default();
        list_of_replacements.push(after);
    }
    (replacements, molecule)
}

fn find_all_distinct_creations(input: &(HashMap<&str, Vec<&str>>, &str)) -> HashSet<String> {
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
    creations
}

fn find_nb_distinct_creations(input: &(HashMap<&str, Vec<&str>>, &str)) -> usize {
    find_all_distinct_creations(input).len()
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

fn nb_steps_to_find_molecule(
    replacements: &HashMap<&str, Vec<&str>>,
    wanted_molecule: &String,
    current_molecule: String,
    nb_steps: u32,
) -> u32 {
    // Apply one step
    let distinct_creations =
        find_all_distinct_creations(&(replacements.clone(), current_molecule.as_str()));

    // Check if molecule found
    if distinct_creations.contains(wanted_molecule) {
        return nb_steps + 1;
    }

    // If not found, check length of creations to stop if they are all too long
    let mut all_too_long = true;
    for creation in distinct_creations.clone() {
        if creation.len() <= wanted_molecule.len() {
            all_too_long = false;
        }
    }
    if all_too_long {
        return 0;
    }

    // Then apply next step to all creations
    let mut all_nb_steps = vec![];
    for mol in distinct_creations {
        all_nb_steps.push(nb_steps_to_find_molecule(
            replacements,
            wanted_molecule,
            mol,
            nb_steps + 1,
        ));
    }
    *all_nb_steps.iter().max().unwrap()
}

fn day19_part2(input: &(HashMap<&str, Vec<&str>>, &str)) {
    // Exemple tests
    let mut replacements_example: HashMap<&str, Vec<&str>> = HashMap::new();
    replacements_example.insert("H", vec!["HO", "OH"]);
    replacements_example.insert("O", vec!["HH"]);
    replacements_example.insert("e", vec!["H", "O"]);
    let molecule_example = "HOH".to_string();
    assert_eq!(
        nb_steps_to_find_molecule(
            &replacements_example,
            &molecule_example,
            String::from("e"),
            0
        ),
        3
    );
    let molecule_example = "HOHOHO".to_string();
    assert_eq!(
        nb_steps_to_find_molecule(
            &replacements_example,
            &molecule_example,
            String::from("e"),
            0
        ),
        6
    );

    // Solve puzzle
    let res = nb_steps_to_find_molecule(&input.0, &input.1.to_string(), String::from("e"), 0);
    println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY19 - part 2: OK!");
}
