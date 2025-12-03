use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY02 -------");
    let example = fs::read_to_string("inputs/example_day02").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day02").expect("Unable to read input!");
    let input = parse(&input);

    day02_part1(&example, &input);
    day02_part2(&example, &input);
}

fn parse(raw_input: &str) -> Vec<(usize, usize)> {
    let mut ranges = vec![];
    let raw_ranges: Vec<&str> = raw_input.split(",").collect();
    for raw_range in raw_ranges {
        let (start, end) = raw_range.split_once("-").unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();
        ranges.push((start, end));
    }
    ranges
}

fn is_valid(id: usize) -> bool {
    let str_id: Vec<char> = format!("{id}").chars().collect();
    let len_id = str_id.len();
    if len_id % 2 != 0 {
        return true;
    }

    let half_len = len_id / 2;
    str_id[..half_len] != str_id[half_len..]
}

fn count_invalid_ids_in_range(range: (usize, usize)) -> usize {
    let (start, end) = range;
    let mut nb = 0;
    for id in start..=end {
        if !is_valid(id) {
            nb += 1;
        }
    }
    nb
}

fn sum_invalid_ids_in_range(range: (usize, usize)) -> usize {
    let (start, end) = range;
    let mut nb = 0;
    for id in start..=end {
        if !is_valid(id) {
            nb += id;
        }
    }
    nb
}

fn solve_part1(ranges: &[(usize, usize)]) -> usize {
    let mut res = 0;
    for &range in ranges {
        res += sum_invalid_ids_in_range(range);
    }
    res
}

fn day02_part1(example: &[(usize, usize)], input: &[(usize, usize)]) {
    // Exemple tests
    assert!(!is_valid(55));
    assert!(!is_valid(6464));
    assert!(!is_valid(123123));
    assert!(is_valid(101));
    assert!(!is_valid(11));
    assert!(!is_valid(22));
    assert!(!is_valid(99));
    assert!(!is_valid(1010));
    assert!(!is_valid(1188511885));
    assert!(!is_valid(222222));
    assert!(!is_valid(446446));
    assert!(!is_valid(38593859));
    assert!(is_valid(1188511880));
    assert_eq!(count_invalid_ids_in_range((11, 22)), 2);
    assert_eq!(count_invalid_ids_in_range((1188511880, 1188511890)), 1);
    assert_eq!(count_invalid_ids_in_range((1698522, 1698528)), 0);
    assert_eq!(solve_part1(example), 1227775554);

    // Solve puzzle
    let res = solve_part1(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 34826702005);
    println!("> DAY02 - part 1: OK!");
}

fn smaller_divisors(n: usize) -> Vec<usize> {
    let mut divisors = vec![];
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            if i != 1 {
                divisors.push(i);
            }
            let other = n / i;
            if i != other {
                divisors.push(other);
            }
        }
        i += 1;
    }
    divisors.sort();
    divisors
}

fn is_valid_hard(id: usize) -> bool {
    let str_id: Vec<char> = format!("{id}").chars().collect();
    let len_id = str_id.len();
    let len_divisors = smaller_divisors(len_id);

    'main_loop: for div in len_divisors {
        let div_len = len_id / div;
        let base = &str_id[..div_len];
        for k in 1..div {
            if *base != str_id[(div_len * k)..(div_len * (k + 1))] {
                continue 'main_loop;
            }
        }
        return false;
    }
    true
}

fn count_invalid_ids_in_range_hard(range: (usize, usize)) -> usize {
    let (start, end) = range;
    let mut nb = 0;
    for id in start..=end {
        if !is_valid_hard(id) {
            nb += 1;
        }
    }
    nb
}

fn sum_invalid_ids_in_range_hard(range: (usize, usize)) -> usize {
    let (start, end) = range;
    let mut nb = 0;
    for id in start..=end {
        if !is_valid_hard(id) {
            nb += id;
        }
    }
    nb
}

fn solve_part2(ranges: &[(usize, usize)]) -> usize {
    let mut res = 0;
    for &range in ranges {
        res += sum_invalid_ids_in_range_hard(range);
    }
    res
}

fn day02_part2(example: &[(usize, usize)], input: &[(usize, usize)]) {
    // Exemple tests
    assert!(is_valid_hard(110));
    assert_eq!(count_invalid_ids_in_range_hard((95, 115)), 2);
    assert_eq!(count_invalid_ids_in_range_hard((1698522, 1698528)), 0);
    assert!(!is_valid_hard(2121212121));
    assert_eq!(count_invalid_ids_in_range_hard((2121212118, 2121212124)), 1);
    assert_eq!(solve_part2(example), 4174379265);

    // Solve puzzle
    let res = solve_part2(input);
    println!("Result part 2: {res}");
    assert_eq!(res, 43287141963);
    println!("> DAY02 - part 2: OK!");
}
