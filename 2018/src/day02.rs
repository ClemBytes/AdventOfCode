use std::collections::HashMap;
use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY02 -------");
    let example = fs::read_to_string("inputs/example_day02").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day02").expect("Unable to read input!");
    let input = parse(&input);

    day02_part1(&example, &input);
    // day02_part2(&example, &input);
}

fn parse(raw_input: &str) -> Vec<&str> {
    let mut boxes = vec![];
    for line in raw_input.lines() {
        boxes.push(line.trim());
    }
    boxes
}

fn letter_appear_n_times(id: &str, n: u8) -> bool {
    let id_chars = id.chars();
    let mut letters_seen: HashMap<char, u8> = HashMap::new();
    for letter in id_chars {
        let count = letters_seen.entry(letter).or_insert(0);
        *count += 1;
    }
    for nb_appear in letters_seen.values() {
        if *nb_appear == n {
            return true;
        }
    }
    false
}

fn checksum(boxes: &Vec<&str>) -> u32 {
    let mut nb_two = 0;
    let mut nb_three = 0;
    for id in boxes {
        if letter_appear_n_times(id, 2) {
            nb_two += 1;
        }
        if letter_appear_n_times(id, 3) {
            nb_three += 1;
        }
    }
    nb_two * nb_three
}

fn day02_part1(example: &Vec<&str>, input: &Vec<&str>) {
    // Exemple test
    assert!(!letter_appear_n_times(example[0], 2));
    assert!(!letter_appear_n_times(example[0], 3));
    assert!(letter_appear_n_times(example[1], 2));
    assert!(letter_appear_n_times(example[1], 3));
    assert!(letter_appear_n_times(example[2], 2));
    assert!(!letter_appear_n_times(example[2], 3));
    assert!(!letter_appear_n_times(example[3], 2));
    assert!(letter_appear_n_times(example[3], 3));
    assert!(letter_appear_n_times(example[4], 2));
    assert!(!letter_appear_n_times(example[4], 3));
    assert!(letter_appear_n_times(example[5], 2));
    assert!(!letter_appear_n_times(example[5], 3));
    assert!(!letter_appear_n_times(example[6], 2));
    assert!(letter_appear_n_times(example[6], 3));
    assert_eq!(checksum(example), 12);
    println!("Example OK");

    // Solve puzzle
    let res = checksum(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 5952);
    println!("> DAY02 - part 1: OK!");
}

// fn day02_part2(_example: &Vec<&str>, _input: &Vec<&str>) {
//     println!("TODO - part2");
//     // Exemple tests
//     // assert_eq!(, 0);
//     // println!("Example OK");
//
//     // Solve puzzle
//     // let res =
//     // println!("Result part 2: {res}");
//     // assert_eq!(res, );
//     // println!("> DAY02 - part 2: OK!");
// }
