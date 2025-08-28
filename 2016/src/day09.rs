use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY09 -------");
    let input = fs::read_to_string("inputs/input_day09").expect("Unable to read input!");
    let input = input.trim();

    day09_part1(input);
    day09_part2(input);
}

fn decompress(input: &str, print: bool) -> usize {
    let mut decompressed_string = String::new();
    let mut i = 0;
    let input_chars: Vec<char> = input.chars().collect();
    while i < input_chars.len() {
        let ch = input_chars[i];
        if ch == '(' {
            i += 1;
            let before = i;
            i += input_chars[i..].iter().position(|x| *x == 'x').unwrap();
            let nb_chars: usize = input[before..i].parse().unwrap();
            i += 1;
            let before = i;
            while input_chars[i] != ')' {
                i += 1;
            }
            let repeat: usize = input[before..i].parse().unwrap();
            for _ in 0..repeat {
                decompressed_string.push_str(&input[i..i + nb_chars]);
            }
            i += nb_chars + 1;
        } else {
            assert_ne!(ch, ' ');
            if ch != ' ' {
                decompressed_string.push(ch);
            }
            i += 1;
        }
    }
    if print {
        println!("{input} => {decompressed_string}");
    }
    decompressed_string.len()
}

fn day09_part1(input: &str) {
    // Exemple tests
    let ex = "ADVENT";
    let res = decompress(ex, true);
    assert_eq!(res, 6);
    let ex = "A(1x5)BC";
    let res = decompress(ex, true);
    assert_eq!(res, 7);
    let ex = "(3x3)XYZ";
    let res = decompress(ex, true);
    assert_eq!(res, 9);
    let ex = "A(2x2)BCD(2x2)EFG";
    let res = decompress(ex, true);
    assert_eq!(res, 11);
    let ex = "(6x1)(1x3)A";
    let res = decompress(ex, true);
    assert_eq!(res, 6);
    let ex = "X(8x2)(3x3)ABCY";
    let res = decompress(ex, true);
    assert_eq!(res, 18);

    // Solve puzzle
    let res = decompress(input, false);
    println!("Result part 1: {res}");
    assert_eq!(res, 102239);
    println!("> DAY09 - part 1: OK!");
}

fn decompress_v2(input: &[char]) -> usize {
    if input.is_empty() {
        return 0;
    }
    if input.len() == 1 {
        return 1;
    }
    let mut i = 0;
    let mut ch = input[i];
    if ch == '(' {
        let mut nb_chars = String::new();
        i += 1;
        ch = input[i];
        while ch != 'x' {
            nb_chars.push(ch);
            i += 1;
            ch = input[i];
        }
        assert_eq!(ch, 'x');
        let nb_chars: usize = nb_chars.parse().unwrap();
        let mut repeat = String::new();
        i += 1;
        ch = input[i];
        while ch != ')' {
            repeat.push(ch);
            i += 1;
            ch = input[i];
        }
        assert_eq!(ch, ')');
        let repeat: usize = repeat.parse().unwrap();
        i += 1;
        repeat * decompress_v2(&input[i..i + nb_chars]) + decompress_v2(&input[i + nb_chars..])
    } else {
        i += 1;
        1 + decompress_v2(&input[i..])
    }
}

fn day09_part2(input: &str) {
    // Exemple tests
    let ex: Vec<char> = "(3x3)XYZ".chars().collect();
    let res = decompress_v2(&ex);
    assert_eq!(res, 9);
    let ex: Vec<char> = "X(8x2)(3x3)ABCY".chars().collect();
    let res = decompress_v2(&ex);
    assert_eq!(res, 20);
    let ex: Vec<char> = "(27x12)(20x12)(13x14)(7x10)(1x12)A".chars().collect();
    let res = decompress_v2(&ex);
    assert_eq!(res, 241920);
    let ex: Vec<char> = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"
        .chars()
        .collect();
    let res = decompress_v2(&ex);
    assert_eq!(res, 445);

    // Solve puzzle
    let input: Vec<char> = input.chars().collect();
    let res = decompress_v2(&input);
    println!("Result part 2: {res}");
    assert_eq!(res, 10780403063);
    println!("> DAY09 - part 2: OK!");
}
