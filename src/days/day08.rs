use std::fs;

pub fn run() {
    println!("------- DAY08 -------");
    let example = fs::read_to_string("inputs/example_day08").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day08").expect("Unable to read input!");
    let input = parse(&input);

    day08_part1(example, input);
    day08_part2(example, input);
}

fn parse(raw_input: &str) -> (i32, i32) {
    let mut nb_chars_code = 0;
    let mut nb_chars_mem = 0;
    for line in raw_input.lines() {
        // Nb characters of code is direct
        nb_chars_code += line.len() as i32;
        // A bit more work for nb characters in memory
        let all_chars: Vec<char> = line.chars().collect();
        let mut i: usize = 0;
        while i < all_chars.len() {
            if all_chars[i] == '\\' {
                if all_chars[i + 1] == '\\' || all_chars[i + 1] == '\"' {
                    nb_chars_mem += 1;
                    i += 2;
                } else if all_chars[i + 1] == 'x' {
                    nb_chars_mem += 1;
                    i += 4;
                }
            } else {
                nb_chars_mem += 1;
                i += 1;
            }
        }
        // Delete opening and closing ""
        nb_chars_mem -= 2;
    }
    (nb_chars_code, nb_chars_mem)
}

fn day08_part1(example: (i32, i32), input: (i32, i32)) {
    // Exemple tests
    // println!("Example input: {example:#?}");
    assert_eq!(example.0 - example.1, 12);

    // Solve puzzle
    // println!("Result part 1: {}", input.0 - input.1);
    assert_eq!(input.0 - input.1, 1350);
    println!("> DAY08 - part 1: OK!");
}

fn day08_part2(_example: (i32, i32), _input: (i32, i32)) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // println!("Result part 2: {}");
    // assert_eq!(, );
    // println!("> DAY08 - part 2: OK!");
}
