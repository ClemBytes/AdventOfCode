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

fn compute_score(input: &str) -> u32 {
    let mut total_score = 0;
    let mut current_score = 0;
    let chars_input: Vec<char> = input.chars().collect();
    let size_input = chars_input.len();
    let mut canceled = false;
    let mut garbage = false;
    for &c in chars_input.iter().take(size_input) {
        if canceled {
            canceled = false;
            continue;
        }
        if garbage {
            if c == '!' {
                canceled = true;
            }
            if c == '>' {
                garbage = false;
            }
            continue;
        }
        match c {
            '{' => current_score += 1,
            '}' => {
                total_score += current_score;
                current_score -= 1;
            }
            '<' => garbage = true,
            '>' => garbage = false,
            '!' => canceled = true,
            _ => continue,
        }
        // println!("position: {position} | character: '{c}' | current_score: {current_score} | total_score: {total_score}")
    }
    total_score
}

fn day09_part1(input: &str) {
    // Exemple tests
    assert_eq!(compute_score("{}"), 1);
    assert_eq!(compute_score("{{{}}}"), 6);
    assert_eq!(compute_score("{{},{}}"), 5);
    assert_eq!(compute_score("{{{},{},{{}}}}"), 16);
    assert_eq!(compute_score("{<a>,<a>,<a>,<a>}"), 1);
    assert_eq!(compute_score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    assert_eq!(compute_score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    assert_eq!(compute_score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);

    // Solve puzzle
    let res = compute_score(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 21037);
    println!("> DAY09 - part 1: OK!");
}

fn count_garbage(input: &str) -> u32 {
    let mut garbage_count = 0;
    let chars_input: Vec<char> = input.chars().collect();
    let size_input = chars_input.len();
    let mut canceled = false;
    let mut garbage = false;
    for &c in chars_input.iter().take(size_input) {
        if canceled {
            canceled = false;
            continue;
        }
        match c {
            '<' => {
                if !garbage {
                    garbage = true;
                    continue;
                }
            }
            '>' => {
                if garbage {
                    garbage = false;
                    continue;
                }
            }
            '!' => {
                canceled = true;
                continue;
            }
            _ => {}
        }
        if garbage {
            garbage_count += 1;
        }
        // println!("position: {position} | character: '{}' | garbage_count: {garbage_count} | garbage: {garbage}", chars_input[position])
    }
    garbage_count
}

fn day09_part2(input: &str) {
    // Exemple tests
    assert_eq!(count_garbage("<>"), 0);
    assert_eq!(count_garbage("<random characters>"), 17);
    assert_eq!(count_garbage("<<<<>"), 3);
    assert_eq!(count_garbage("<{!>}>"), 2);
    assert_eq!(count_garbage("<!!>"), 0);
    assert_eq!(count_garbage("<!!!>>"), 0);
    assert_eq!(count_garbage("<{o\"i!a,<{i<a>"), 10);

    // Solve puzzle
    let res = count_garbage(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 9495);
    println!("> DAY09 - part 2: OK!");
}
