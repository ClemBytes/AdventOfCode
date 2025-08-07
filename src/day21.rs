use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY21 -------");
    let input = fs::read_to_string("inputs/input_day21").expect("Unable to read input!");
    let boss = parse(&input);

    let player = Person {
        hit_points: 100,
        damage: 0,
        armor: 0,
    };

    let shop = Shop {
        weapons: vec![
            Item::Weapon(8, 4, 0),
            Item::Weapon(10, 5, 0),
            Item::Weapon(25, 6, 0),
            Item::Weapon(40, 7, 0),
            Item::Weapon(74, 8, 0),
        ],
        armors: vec![
            Item::Armor(13, 0, 1),
            Item::Armor(31, 0, 2),
            Item::Armor(53, 0, 3),
            Item::Armor(75, 0, 4),
            Item::Armor(102, 0, 5),
        ],
        rings: vec![
            Item::Ring(25, 1, 0),
            Item::Ring(50, 2, 0),
            Item::Ring(100, 3, 0),
            Item::Ring(20, 0, 1),
            Item::Ring(40, 0, 2),
            Item::Ring(80, 0, 3),
        ],
    };

    day21_part1(boss.clone(), player.clone(), &shop);
    day21_part2(boss, player, &shop);
}

#[derive(Clone)]
struct Person {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

enum Item {
    // Player must buy exactly 1 weapon
    Weapon(i32, i32, i32),
    // Player can buy 0 or 1 armor
    Armor(i32, i32, i32),
    // Player can buy 0, 1 or 2 rings
    Ring(i32, i32, i32),
}

struct Shop {
    weapons: Vec<Item>,
    armors: Vec<Item>,
    rings: Vec<Item>,
}

fn parse(raw_input: &String) -> Person {
    let lines: Vec<&str> = raw_input.split("\n").collect();
    Person { 
        hit_points: lines[0].split(": ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap(),
        damage: lines[1].split(": ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap(),
        armor: lines[2].split(": ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap(),
    }
}

fn play(boss: Person, player: Person) -> bool {
    // Return true if player wins, false otherwise
    let mut boss = boss.clone();
    let mut player = player.clone();
    let mut player_turn = true;
    while boss.hit_points > 0 && player.hit_points > 0 {
        if player_turn {
            boss.hit_points -= (player.damage - boss.armor).max(1);
            player_turn = false;
        } else {
            player.hit_points -= (boss.damage - player.armor).max(1);
            player_turn = true;
        }
    }
    player.hit_points > boss.hit_points
}

fn day21_part1(boss: Person, player: Person, _shop: &Shop) {
    // Exemple tests
    let mut boss_example = boss.clone();
    boss_example.hit_points = 12;
    boss_example.damage = 7;
    boss_example.armor = 2;
    let mut player_example = player.clone();
    player_example.hit_points = 8;
    player_example.damage = 5;
    player_example.armor = 5;
    assert!(play(boss_example, player_example));

    // Solve puzzle
    // let res = 
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY21 - part 1: OK!");
}

fn day21_part2(_boss: Person, _player: Person, _shop: &Shop) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res = 
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY21 - part 2: OK!");
}
