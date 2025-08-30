#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY19 -------");
    let example = 5;
    let input = 3004953;

    day19_part1(example, input);
    day19_part2(example, input);
}

fn solve_part1(nb_elves: u32) -> u32 {
    // Create circle
    let mut circle: Vec<u32> = (1..(nb_elves + 1)).collect();
    while circle.len() > 2 {
        let mut new_circle = vec![];
        if circle.len() % 2 == 0 {
            new_circle.push(circle[0]);
        }
        for (i, &val) in circle.iter().enumerate().skip(2) {
            if i % 2 == 0 {
                new_circle.push(val);
            }
        }
        circle.clear();
        circle = new_circle;
    }
    circle[0]
}

fn day19_part1(example: u32, input: u32) {
    // Exemple tests
    assert_eq!(solve_part1(example), 3);
    assert_eq!(solve_part1(10), 5);
    println!("Example OK");

    // Solve puzzle
    let res = solve_part1(input);
    println!("Result part 1: {res}");
    assert_eq!(res, 1815603);
    println!("> DAY19 - part 1: OK!");
}

fn day19_part2(_example: u32, _input: u32) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);
    // println!("Example OK");

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY19 - part 2: OK!");
}
