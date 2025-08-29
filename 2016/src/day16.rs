#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY16 -------");
    let input = "01111001100111011";

    day16_part1(input);
    day16_part2(input);
}

fn dragon_step(input: &str) -> String {
    let b: String = input.chars().rev().collect();
    let mut res = input.to_owned();
    res.push('0');
    for ch in b.chars() {
        let add = match ch {
            '0' => '1',
            '1' => '0',
            _ => panic!("Character should be '0' or '1' not '{ch}'"),
        };
        res.push(add);
    }
    res
}

fn fill_disk(input: &str, size_disk: usize) -> String {
    let mut res = input.to_owned();
    while res.len() < size_disk {
        res = dragon_step(&res);
    }
    res[..size_disk].to_string()
}

fn checksum(dragon_result: &str) -> String {
    let mut res = String::new();
    let mut checksum: Vec<char> = dragon_result.to_owned().chars().collect();
    while res.len() % 2 == 0 {
        res.clear();
        for i in 0..(checksum.len() / 2) {
            let add = match checksum[2 * i] == checksum[2 * i + 1] {
                true => '1',
                false => '0',
            };
            res.push(add);
        }
        checksum = res.chars().collect();
    }
    res
}

fn solve_part1(input: &str, size_disk: usize) -> String {
    checksum(&fill_disk(input, size_disk))
}

fn day16_part1(input: &str) {
    // Exemple tests
    assert_eq!(dragon_step("1"), "100");
    assert_eq!(dragon_step("0"), "001");
    assert_eq!(dragon_step("11111"), "11111000000");
    assert_eq!(dragon_step("111100001010"), "1111000010100101011110000");
    assert_eq!(fill_disk("10000", 20), "10000011110010000111");
    assert_eq!(checksum("110010110100"), "100");
    assert_eq!(solve_part1("10000", 20), "01100");

    // Solve puzzle
    let res = solve_part1(input, 272);
    println!("Result part 1: {res}");
    assert_eq!(res, "11111000111110000");
    println!("> DAY16 - part 1: OK!");
}

fn day16_part2(_input: &str) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY16 - part 2: OK!");
}
