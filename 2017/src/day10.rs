use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY10 -------");
    let example = vec![3, 4, 1, 5];
    let input = fs::read_to_string("inputs/input_day10").expect("Unable to read input!");
    let input = parse(&input);

    day10_part1(&example, &input);
    day10_part2(&example, &input);
}

fn parse(raw_input: &str) -> Vec<usize> {
    raw_input
        .trim()
        .split(",")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn apply_one_turn(list: Vec<usize>, length: usize, current_position: usize) -> Vec<usize> {
    let list_len = list.len();
    let mut res = list.clone();
    let position_end_section = current_position + length;
    for i in current_position..position_end_section {
        let mirror_position = (position_end_section - (i - current_position) - 1) % list_len;
        // let mirror_position = (position_end_section - i - 1) % list_len;
        // println!("i: {i} | mirror_position: {mirror_position}");
        res[i % list_len] = list[mirror_position];
        res[mirror_position] = list[i % list_len];
    }
    // println!("current_position: {current_position} | position_end_section: {position_end_section}");
    res
}

fn hash(size_list: usize, lengths: &[usize]) -> usize {
    let mut list: Vec<usize> = (0..size_list).collect();
    let mut current_position = 0;
    for (skip_size, &length) in lengths.iter().enumerate() {
        list = apply_one_turn(list, length, current_position);
        current_position += (length + skip_size) % size_list;
    }
    list[0] * list[1]
}

fn day10_part1(example: &[usize], input: &[usize]) {
    // Exemple tests
    let e = vec![0, 1, 2, 3, 4];
    // println!("{e:?}");
    let e1 = apply_one_turn(e, 3, 0);
    assert_eq!(e1, vec![2, 1, 0, 3, 4]);
    let e2 = apply_one_turn(e1, 4, 3);
    assert_eq!(e2, vec![4, 3, 0, 1, 2]);
    let e3 = apply_one_turn(e2, 1, 3);
    assert_eq!(e3, vec![4, 3, 0, 1, 2]);
    let e4 = apply_one_turn(e3, 5, 1);
    assert_eq!(e4, vec![3, 4, 2, 1, 0]);
    assert_eq!(hash(5, example), 12);

    // Solve puzzle
    let res = hash(256, input);
    println!("Result part 1: {res}");
    assert_eq!(res, 11413);
    println!("> DAY10 - part 1: OK!");
}

fn day10_part2(_example: &[usize], _input: &[usize]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY10 - part 2: OK!");
}
