use std::collections::HashMap;
use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY02 -------");
    let example1 = fs::read_to_string("inputs/example_day02_part1").expect("Unable to read input!");
    let example1 = parse(&example1);
    let example2 = fs::read_to_string("inputs/example_day02_part2").expect("Unable to read input!");
    let example2 = parse(&example2);
    let input = fs::read_to_string("inputs/input_day02").expect("Unable to read input!");
    let input = parse(&input);

    day02_part1(&example1, &input);
    day02_part2(&example2, &input);
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

fn distance(id1: &str, id2: &str) -> u8 {
    assert_eq!(id1.len(), id2.len());
    let id1_chars = id1.chars();
    let id2_chars: Vec<char> = id2.chars().collect::<Vec<char>>();
    let mut dist = 0;
    for (i, l1) in id1_chars.enumerate() {
        if l1 != id2_chars[i] {
            dist += 1;
        }
    }
    dist
}

fn correct_id(boxes: &[&str]) -> String {
    let nb_boxes = boxes.len();
    for i in 0..nb_boxes {
        for j in i + 1..nb_boxes {
            if distance(boxes[i], boxes[j]) == 1 {
                let mut res = String::new();
                let id1_chars = boxes[i].chars();
                let id2_chars: Vec<char> = boxes[j].chars().collect::<Vec<char>>();
                for (i, l1) in id1_chars.enumerate() {
                    if l1 == id2_chars[i] {
                        res += &l1.to_string();
                    }
                }
                return res;
            }
        }
    }
    unreachable!();
}

fn day02_part2(example: &Vec<&str>, input: &Vec<&str>) {
    // Exemple tests
    assert_eq!(distance(example[0], example[5]), 2);
    assert_eq!(distance(example[1], example[4]), 1);
    assert_eq!(correct_id(example), "fgij".to_string());
    println!("Example OK");

    // Solve puzzle
    let res = correct_id(input);
    println!("Result part 2: {res}");
    assert_eq!(res, "krdmtuqjgwfoevnaboxglzjph".to_string());
    println!("> DAY02 - part 2: OK!");
}
