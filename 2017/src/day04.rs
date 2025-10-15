use std::{collections::HashSet, fs};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY04 -------");
    // let example = fs::read_to_string("inputs/example_day04").expect("Unable to read input!");
    // let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day04").expect("Unable to read input!");
    let input = parse(&input);

    day04_part1(&input);
    day04_part2(&input);
}

fn parse(raw_input: &str) -> Vec<String> {
    let mut passwords = vec![];
    for line in raw_input.lines() {
        passwords.push(line.to_string());
    }
    passwords
}

fn nb_valid_passwords(passwords: &Vec<String>) -> u32 {
    let mut nb = 0;
    for password in passwords {
        let words: Vec<&str> = password.split_whitespace().collect();
        let mut words_set = HashSet::new();
        for &word in &words {
            words_set.insert(word);
        }
        if words.len() == words_set.len() {
            nb += 1
        }
    }
    nb
}

fn day04_part1(input: &Vec<String>) {
    // Solve puzzle
    let res = nb_valid_passwords(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 466);
    println!("> DAY04 - part 1: OK!");
}

fn nb_valid_passwords_without_anagrams(passwords: &Vec<String>) -> u32 {
    let mut nb = 0;
    for password in passwords {
        let words: Vec<&str> = password.split_whitespace().collect();
        let mut words_set = HashSet::new();
        let mut words_list = vec![];
        for &word in &words {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort_unstable();
            let sorted_word: String = chars.into_iter().collect();
            words_set.insert(sorted_word.clone());
            words_list.push(sorted_word);
        }
        if words_list.len() == words_set.len() {
            nb += 1
        }
    }
    nb
}

fn day04_part2(input: &Vec<String>) {
    // Solve puzzle
    let res = nb_valid_passwords_without_anagrams(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 251);
    println!("> DAY04 - part 2: OK!");
}
