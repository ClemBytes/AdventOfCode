use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY05 -------");
    let example = fs::read_to_string("inputs/example_day05").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day05").expect("Unable to read input!");
    let input = parse(&input);

    day05_part1(&example, &input);
    day05_part2(&example, &input);
}

fn parse(raw_input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut ranges = vec![];
    let mut ids = vec![];
    let mut is_range = true;
    for line in raw_input.lines() {
        if line.is_empty() {
            is_range = false;
            continue;
        }
        if is_range {
            let (start, end) = line.split_once("-").unwrap();
            ranges.push((start.parse().unwrap(), end.parse().unwrap()));
        } else {
            ids.push(line.parse().unwrap());
        }
    }
    (ranges, ids)
}

fn is_in_ranges(id: usize, ranges: &[(usize, usize)]) -> bool {
    for range in ranges {
        if (id >= range.0) && (id <= range.1) {
            return true;
        }
    }
    false
}

fn count_fresh_ids(input: &(Vec<(usize, usize)>, Vec<usize>)) -> usize {
    let mut nb_fresh = 0;
    let ranges = &input.0;
    for id in &input.1 {
        if is_in_ranges(*id, ranges) {
            nb_fresh += 1;
        }
    }
    nb_fresh
}

fn day05_part1(
    example: &(Vec<(usize, usize)>, Vec<usize>),
    input: &(Vec<(usize, usize)>, Vec<usize>),
) {
    // Exemple tests
    assert!(!is_in_ranges(1, &[(3, 5)]));
    assert!(is_in_ranges(3, &[(3, 5)]));
    assert!(is_in_ranges(4, &[(3, 5)]));
    assert!(is_in_ranges(5, &[(3, 5)]));
    assert!(!is_in_ranges(1, &example.0));
    assert!(is_in_ranges(5, &example.0));
    assert!(!is_in_ranges(8, &example.0));
    assert!(is_in_ranges(11, &example.0));
    assert!(is_in_ranges(17, &example.0));
    assert!(!is_in_ranges(32, &example.0));
    assert_eq!(count_fresh_ids(example), 3);

    // Solve puzzle
    let res = count_fresh_ids(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 735);
    println!("> DAY05 - part 1: OK!");
}

fn day05_part2(
    _example: &(Vec<(usize, usize)>, Vec<usize>),
    _input: &(Vec<(usize, usize)>, Vec<usize>),
) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY05 - part 2: OK!");
}
