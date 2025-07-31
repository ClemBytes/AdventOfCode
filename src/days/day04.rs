pub fn run() {
    println!("------- DAY04 -------");
    let input = "bgvyzdsv";
    day04_part1(input);
    day04_part2(input);
}

fn day04_part1(input: &str) {
    let start = "00000";
    // Exemple tests
    assert_eq!(find_suffix("abcdef", start), 609043);
    assert_eq!(find_suffix("pqrstuv", start), 1048970);

    // Solve puzzle
    // println!("Result part 1: {}", find_suffix(input, start));
    assert_eq!(find_suffix(input, start), 254575);
    println!("> DAY04 - part 1: OK!");
}

fn day04_part2(input: &str) {
    let start = "000000";
    // Solve puzzle
    // println!("Result part 2: {}", find_suffix(input, start));
    assert_eq!(find_suffix(input, start), 1038736);
    println!("> DAY04 - part 2: OK!");
}

fn find_suffix(secret_key: &str, start: &str) -> u32 {
    let mut i = 1;
    loop {
        let s = format!("{secret_key}{i}");
        let hash = format!("{:x}", md5::compute(s));
        if hash.starts_with(start) {
            return i;
        }
        i += 1;
    }
}
