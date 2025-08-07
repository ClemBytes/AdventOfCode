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
            Item{item_type: ItemType::Weapon, cost: 8, damage: 4, armor: 0},
            Item{item_type: ItemType::Weapon, cost: 10, damage: 5, armor: 0},
            Item{item_type: ItemType::Weapon, cost: 25, damage: 6, armor: 0},
            Item{item_type: ItemType::Weapon, cost: 40, damage: 7, armor: 0},
            Item{item_type: ItemType::Weapon, cost: 74, damage: 8, armor: 0},
        ],
        armors: vec![
            Item{item_type: ItemType::Armor, cost: 13, damage: 0, armor: 1},
            Item{item_type: ItemType::Armor, cost: 31, damage: 0, armor: 2},
            Item{item_type: ItemType::Armor, cost: 53, damage: 0, armor: 3},
            Item{item_type: ItemType::Armor, cost: 75, damage: 0, armor: 4},
            Item{item_type: ItemType::Armor, cost: 102, damage: 0, armor: 5},
        ],
        rings: vec![
            Item{item_type: ItemType::Ring, cost: 25, damage: 1, armor: 0},
            Item{item_type: ItemType::Ring, cost: 50, damage: 2, armor: 0},
            Item{item_type: ItemType::Ring, cost: 100, damage: 3, armor: 0},
            Item{item_type: ItemType::Ring, cost: 20, damage: 0, armor: 1},
            Item{item_type: ItemType::Ring, cost: 40, damage: 0, armor: 2},
            Item{item_type: ItemType::Ring, cost: 80, damage: 0, armor: 3},
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

enum ItemType {
    // Player must buy exactly 1 weapon
    Weapon,
    // Player can buy 0 or 1 armor
    Armor,
    // Player can buy 0, 1 or 2 rings
    Ring,
}

struct Item {
    item_type: ItemType,
    cost: i32,
    damage: i32,
    armor: i32,
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

fn find_cheapest_win(boss: Person, player: Person, shop: &Shop) -> i32 {
    // Generate all possible layouts for the player with:
    // - Exactly 1 weapon
    // - 0 or 1 armor
    // - 0, 1 or 2 rings
    // Each player is associated with its total cost
    let mut layouts: Vec<(Person, i32)> = vec![];
    for weapon in shop.weapons.iter() {
        let weapon: Item::Weapon = &weapon;
        let new_player = player.clone();
        new_player.damage += weapon.damage;
    }
    0
}

fn day21_part1(boss: Person, player: Person, shop: &Shop) {
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
    let res = find_cheapest_win(boss, player, shop);
    println!("Result part 1: {res}");
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
