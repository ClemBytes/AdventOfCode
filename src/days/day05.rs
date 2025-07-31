// DAY05: https://adventofcode.com/2015/day/5

use std::fs::File;
use std::io::{self, BufRead};

pub fn run() {
    println!("------- DAY05 -------");
    let path = "inputs/input_day05";
    day05_part1(path);
    day05_part2(path);
}

fn day05_part1(path: &str) {
    // Exemple tests
    assert!(is_nice("ugknbfddgicrmopn"));
    assert!(!is_nice("haegwjzuvuyypxyu"));
    assert!(!is_nice("dvszwmarrgswjxmb"));
    assert!(!is_nice("jchzalrnumimnmhp"));

    // Solve puzzle
    let mut counter = 0;
    if let Ok(file) = File::open(path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if is_nice(&line.unwrap()) {
                counter += 1;
            }
        }
    }
    // println!("Result part 1: {counter}");
    assert_eq!(counter, 236);
    println!("> DAY05 - part 1: OK!");
}

fn day05_part2(path: &str) {
    // Exemple tests
    assert!(new_is_nice("qjhvhtzxzqqjkmpb"));
    assert!(new_is_nice("xxyxx"));
    assert!(!new_is_nice("uurcxstgmygtbstg"));
    assert!(!new_is_nice("ieodomkazucvgmuy"));

    // Solve puzzle
    let mut counter = 0;
    if let Ok(file) = File::open(path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if new_is_nice(&line.unwrap()) {
                counter += 1;
            }
        }
    }
    // println!("Result part 2: {counter}");
    assert_eq!(counter, 51);
    println!("> DAY05 - part 2: OK!");
}

fn is_nice(s: &str) -> bool {
    // Check forbidden substrings
    let forbidden_substrings = ["ab", "cd", "pq", "xy"];
    for forbidden in forbidden_substrings {
        if s.contains(forbidden) {
            return false;
        }
    }

    // Check at least 3 vowels
    let vowels = "aeiou";
    let mut nb_vowels = 0;
    for ch in s.chars() {
        if vowels.contains(ch) {
            nb_vowels += 1;
        }
    }
    if nb_vowels < 3 {
        return false;
    }

    // Check double letter
    let chars: Vec<char> = s.chars().collect();
    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            return true;
        }
    }
    false
}

fn new_is_nice(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    // Check double double-letter
    let mut double_double = false;
    'main_loop: for i in 1..(chars.len() - 2) {
        for j in (i + 2)..chars.len() {
            if chars[i - 1] == chars[j - 1] && chars[i] == chars[j] {
                double_double = true;
                break 'main_loop;
            }
        }
    }

    if !double_double {
        return false;
    }

    // Check xyx pattern
    for i in 2..chars.len() {
        if chars[i] == chars[i - 2] {
            return true;
        }
    }
    false
}
