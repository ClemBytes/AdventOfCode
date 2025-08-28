// DAY04: https://adventofcode.com/2015/day/4

use md5::Digest;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY04 -------");
    let input = "bgvyzdsv";
    day04_part1(input);
    day04_part2(input);
}

fn has_five_hex_zeros_prefix(hash: Digest) -> bool {
    hash[0] == 0 && hash[1] == 0 && hash[2] & 0xF0 == 0
}

fn day04_part1(input: &str) {
    let find_suffix_5zeroes = |key: &str| find_suffix(key, has_five_hex_zeros_prefix);

    // Exemple tests
    assert_eq!(find_suffix_5zeroes("abcdef"), 609043);
    assert_eq!(find_suffix_5zeroes("pqrstuv"), 1048970);

    // Solve puzzle
    // println!("Result part 1: {}", find_suffix_5zeroes(input));
    assert_eq!(find_suffix_5zeroes(input), 254575);
    println!("> DAY04 - part 1: OK!");
}

fn has_six_hex_zeros_prefix(hash: Digest) -> bool {
    hash[0] == 0 && hash[1] == 0 && hash[2] == 0
}

fn day04_part2(input: &str) {
    let find_suffix_6zeroes = |key: &str| find_suffix(key, has_six_hex_zeros_prefix);

    // Solve puzzle
    // println!("Result part 2: {}", find_suffix_6zeroes(input));
    assert_eq!(find_suffix_6zeroes(input), 1038736);
    println!("> DAY04 - part 2: OK!");
}

fn find_suffix<F: Fn(Digest) -> bool>(secret_key: &str, f: F) -> u32 {
    let mut i = 1;
    loop {
        let s = format!("{secret_key}{i}");
        let hash = md5::compute(s);
        if f(hash) {
            return i;
        }
        i += 1;
    }
}
