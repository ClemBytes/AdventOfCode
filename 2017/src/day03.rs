#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY03 -------");
    let input = 361527;

    day03_part1(input);
    day03_part2(input);
}

fn manhattan_distance(_square: u32) -> u32 {
    0
}

fn day03_part1(input: u32) {
    // Exemple tests
    assert_eq!(manhattan_distance(1), 0);
    assert_eq!(manhattan_distance(12), 3);
    assert_eq!(manhattan_distance(23), 2);
    assert_eq!(manhattan_distance(1024), 31);
    println!("Example OK");

    // Solve puzzle
    let res = manhattan_distance(input);
    println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY03 - part 1: OK!");
}

fn day03_part2(_input: u32) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res = 
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY03 - part 2: OK!");
}
