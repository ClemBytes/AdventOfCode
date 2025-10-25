use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY09 -------");
    let input = fs::read_to_string("inputs/input_day09").expect("Unable to read input!");

    day09_part1(&input);
    day09_part2(&input);
}

fn compute_score (input: &String) -> u32 {
    let mut total_score = 0;
    let mut current_score = 0;
    let chars_input: Vec<char> = input.chars().collect();
    let size_input = chars_input.len();
    let mut canceled = false;
    let mut garbage = false;
    for position in 0..size_input {
        if canceled {
            canceled = false;
            continue;
        }
        if garbage {
            if chars_input[position] == '!' {
                canceled = true;
            }
            if chars_input[position] == '>' {
                garbage = false;
            }
            continue;
        }
        match chars_input[position] {
            '{' => current_score += 1,
            '}' => {
                total_score += current_score;
                current_score -= 1;
            },
            '<' => garbage = true,
            '>' => garbage = false,
            '!' => canceled = true,
            _ => continue
        }
        // println!("position: {position} | character: '{}' | current_score: {current_score} | total_score: {total_score}", chars_input[position])
    }
    total_score
}

fn day09_part1(input: &String) {
    // Exemple tests
    assert_eq!(compute_score(&"{}".to_string()), 1);
    assert_eq!(compute_score(&"{{{}}}".to_string()), 6);
    assert_eq!(compute_score(&"{{},{}}".to_string()), 5);
    assert_eq!(compute_score(&"{{{},{},{{}}}}".to_string()), 16);
    assert_eq!(compute_score(&"{<a>,<a>,<a>,<a>}".to_string()), 1);
    assert_eq!(compute_score(&"{{<ab>},{<ab>},{<ab>},{<ab>}}".to_string()), 9);
    assert_eq!(compute_score(&"{{<!!>},{<!!>},{<!!>},{<!!>}}".to_string()), 9);
    assert_eq!(compute_score(&"{{<a!>},{<a!>},{<a!>},{<ab>}}".to_string()), 3);

    // Solve puzzle
    let res = compute_score(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 21037);
    println!("> DAY09 - part 1: OK!");
}

fn day09_part2(_input: &String) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY09 - part 2: OK!");
}
