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

fn parse(raw_input: &str) -> Vec<u32> {
    raw_input.trim().split(",").collect::<Vec<&str>>().into_iter().map(|x| x.parse::<u32>().unwrap()).collect()
}

fn apply_one_turn(list: Vec<u32>, length: usize, current_position: usize) -> Vec<u32> {
    let list_len = list.len();
    let mut res = list.clone();
    let position_end_section = (current_position + length) % list_len;
    for i in current_position..position_end_section {
        let mirror_position = (position_end_section - i) % list_len;
        res[i] = list[mirror_position];
        res[mirror_position] = list[i];
    }
    res
}

fn day10_part1(_example: &Vec<u32>, _input: &Vec<u32>) {
    // Exemple tests
    let e = vec![0, 1, 2, 3, 4];
    println!("{e:?}");
    let e1 = apply_one_turn(e, 3, 0);
    println!("{e1:?}");
    let e2 = apply_one_turn(e1, 4, 3);
    println!("{e2:?}");
    let e3 = apply_one_turn(e2, 1, 3);
    println!("{e3:?}");
    let e4 = apply_one_turn(e3, 5, 1);
    println!("{e4:?}");
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY10 - part 1: OK!");
}

fn day10_part2(_example: &Vec<u32>, _input: &Vec<u32>) {
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
