use std::{collections::HashSet, fs};

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

fn count_all_fresh_ids_naive(ranges: &[(usize, usize)]) -> usize {
    let mut fresh_ids = HashSet::new();
    for range in ranges {
        let start = range.0;
        let end = range.1;
        for id in start..=end {
            fresh_ids.insert(id);
        }
    }
    fresh_ids.len()
}

fn count_all_fresh_ids(ranges: &[(usize, usize)]) -> usize {
    // Merge ranges
    // Sort ranges by start points
    let mut ranges = ranges.to_vec();
    ranges.sort();

    // Then merge touching of overlapping intervals
    let mut ranges_unions = vec![];
    let mut current_range = ranges[0];
    for (i, next_range) in ranges.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if next_range.0 <= current_range.1 {
            current_range.1 = current_range.1.max(next_range.1);
        } else {
            ranges_unions.push(current_range);
            current_range = *next_range;
        }
    }
    ranges_unions.push(current_range);

    // Count fresh ids
    let mut res = 0;
    for range in ranges_unions {
        res += range.1 - range.0 + 1;
    }
    res
}

fn day05_part2(
    example: &(Vec<(usize, usize)>, Vec<usize>),
    input: &(Vec<(usize, usize)>, Vec<usize>),
) {
    // Exemple tests
    assert_eq!(count_all_fresh_ids_naive(&example.0), 14);
    assert_eq!(count_all_fresh_ids(&example.0), 14);

    // Solve puzzle
    let res = count_all_fresh_ids(&input.0);
    println!("Result part 2: {res}");
    assert_eq!(res, 344306344403172);
    println!("> DAY05 - part 2: OK!");
}
