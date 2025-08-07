use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY22 -------");
    let input = fs::read_to_string("inputs/input_day22").expect("Unable to read input!");
    let boss = parse(&input);

    let player = Player {
        hit_points: 50,
        mana: 500,
        armor: 0,
        shield_effect: 0,
        recharge_effect: 0,
    };

    day22_part1(boss.clone(), player.clone());
    day22_part2(boss, player);
}

#[derive(Clone, Debug)]
struct Boss {
    hit_points: i32,
    damage: i32,
    // Poison deals 3 damages to the boss every turn for 6 turns
    poison_effect: i32, // Nb of turns where this effect will still be active
}

#[derive(Clone, Debug)]
struct Player {
    hit_points: i32,
    mana: i32,
    armor: i32,
    // Shield increases player's armor by 7 during 6 turns
    shield_effect: i32, // Nb of turns where this effect will still be active
    // Recharge increases player's mana by 101 every turn for 5 turns
    recharge_effect: i32, // Nb of turns where this effect will still be active
}

fn parse(raw_input: &str) -> Boss {
    let (hit_points, damage) = raw_input.split_once("\n").unwrap();
    Boss {
        hit_points: hit_points.split(": ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap(),
        damage: damage.split(": ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap(),
        poison_effect: 0,
    }
}

fn day22_part1(_boss: Boss, _player: Player) {
    println!("TODO - part1");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY22 - part 1: OK!");
}

fn day22_part2(_boss: Boss, _player: Player) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY22 - part 2: OK!");
}
