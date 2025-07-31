pub fn run() {
    println!("------- DAY04 -------");
    let input = "bgvyzdsv";
    day04_part1(input);
    day04_part2(input);
}

fn day04_part1(input: &str) {
    // Exemple tests
    assert_eq!(find_suffix_5zeroes("abcdef"), 609043);
    assert_eq!(find_suffix_5zeroes("pqrstuv"), 1048970);

    // Solve puzzle
    // println!("Result part 1: {}", find_suffix_5zeroes(input));
    assert_eq!(find_suffix_5zeroes(input), 254575);
    println!("> DAY04 - part 1: OK!");
}

fn day04_part2(input: &str) {
    // Solve puzzle
    // println!("Result part 2: {}", find_suffix_6zeroes(input));
    assert_eq!(find_suffix_6zeroes(input), 1038736);
    println!("> DAY04 - part 2: OK!");
}

fn find_suffix_5zeroes(secret_key: &str) -> u32 {
    let mut i = 1;
    loop {
        let s = format!("{secret_key}{i}");
        let hash = md5::compute(s);
        if hash[0] == 0 && hash[1] == 0 && hash[2] & 0xF0 == 0 {
            return i;
        }
        i += 1;
    }
}

fn find_suffix_6zeroes(secret_key: &str) -> u32 {
    let mut i = 1;
    loop {
        let s = format!("{secret_key}{i}");
        let hash = md5::compute(s);
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return i;
        }
        i += 1;
    }
}
