use std::collections::HashMap;

use md5::Digest;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY05 -------");
    let input = "ugkcyxxp";

    day05_part1(input);
    day05_part2(input);
}

fn has_five_hex_zeros(hash: Digest) -> bool {
    hash[0] == 0 && hash[1] == 0 && hash[2] & 0xF0 == 0
}

fn find_password_part1(door_id: &str) -> String {
    let mut i = 0;
    let mut password = String::new();
    while password.len() < 8 {
        let s = format!("{door_id}{i}");
        let hash = md5::compute(s);
        if hash.len() > 5 && has_five_hex_zeros(hash) {
            password = format!("{password}{:x}", hash[2]);
        }
        i += 1
    }
    password
}

fn day05_part1(input: &str) {
    // Exemple tests
    let example = "abc";
    let res = find_password_part1(example);
    assert_eq!(res, "18f47a30");
    println!();

    // Solve puzzle
    let res = find_password_part1(input);
    println!("Result part 1: {res}");
    assert_eq!(res, "d4cd2ee1");
    println!("> DAY05 - part 1: OK!");
}

fn find_password_part2(door_id: &str) -> String {
    let mut i = 0;
    let mut password_hashset: HashMap<u8, u8> = HashMap::new();
    while password_hashset.len() < 8 {
        let s = format!("{door_id}{i}");
        let hash = md5::compute(s);
        if hash.len() > 5 && has_five_hex_zeros(hash) {
            let position = hash[2];
            let char_in_position = (hash[3] & 0xF0) / 16;
            if position < 8 && !password_hashset.contains_key(&position) {
                password_hashset.insert(position, char_in_position);
            }
        }
        i += 1
    }
    let mut password = String::new();
    for i in 0..8 {
        password = format!("{password}{:x}", password_hashset[&i]);
    }
    password
}

fn day05_part2(input: &str) {
    let example = "abc";
    let res = find_password_part2(example);
    assert_eq!(res, "05ace8e3");

    // Solve puzzle
    let res = find_password_part2(input);
    println!("Result part 2: {res}");
    assert_eq!(res, "f2c730e5");
    println!("> DAY05 - part 2: OK!");
}
