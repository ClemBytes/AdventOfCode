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
            Item {
                cost: 8,
                damage: 4,
                armor: 0,
            },
            Item {
                cost: 10,
                damage: 5,
                armor: 0,
            },
            Item {
                cost: 25,
                damage: 6,
                armor: 0,
            },
            Item {
                cost: 40,
                damage: 7,
                armor: 0,
            },
            Item {
                cost: 74,
                damage: 8,
                armor: 0,
            },
        ],
        armors: vec![
            Item {
                cost: 13,
                damage: 0,
                armor: 1,
            },
            Item {
                cost: 31,
                damage: 0,
                armor: 2,
            },
            Item {
                cost: 53,
                damage: 0,
                armor: 3,
            },
            Item {
                cost: 75,
                damage: 0,
                armor: 4,
            },
            Item {
                cost: 102,
                damage: 0,
                armor: 5,
            },
        ],
        rings: vec![
            Item {
                cost: 25,
                damage: 1,
                armor: 0,
            },
            Item {
                cost: 50,
                damage: 2,
                armor: 0,
            },
            Item {
                cost: 100,
                damage: 3,
                armor: 0,
            },
            Item {
                cost: 20,
                damage: 0,
                armor: 1,
            },
            Item {
                cost: 40,
                damage: 0,
                armor: 2,
            },
            Item {
                cost: 80,
                damage: 0,
                armor: 3,
            },
        ],
    };

    day21_part1(boss.clone(), player.clone(), &shop);
    day21_part2(boss, player, &shop);
}

#[derive(Clone, Debug)]
struct Person {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

struct Shop {
    weapons: Vec<Item>,
    armors: Vec<Item>,
    rings: Vec<Item>,
}

fn parse(raw_input: &str) -> Person {
    let lines: Vec<&str> = raw_input.split("\n").collect();
    Person {
        hit_points: lines[0].split(": ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap(),
        damage: lines[1].split(": ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap(),
        armor: lines[2].split(": ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap(),
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
    // Player must buy exactly 1 weapon
    for weapon in shop.weapons.iter() {
        let mut player_with_weapon = player.clone();
        player_with_weapon.damage += weapon.damage;
        let mut new_cost = weapon.cost;
        // Player can buy 0 armor and 0 ring
        layouts.push((player_with_weapon.clone(), new_cost));
        for armor in shop.armors.iter() {
            // Player now chooses 1 armor
            let mut player_with_weapon_and_armor = player_with_weapon.clone();
            player_with_weapon_and_armor.armor += armor.armor;
            let mut new_cost = new_cost + armor.cost;
            // Player can buy 0 ring
            layouts.push((player_with_weapon_and_armor.clone(), new_cost));
            for first_ring in shop.rings.iter() {
                // Player now chooses 1 first ring
                let mut player_with_weapon_and_armor_and_1_ring =
                    player_with_weapon_and_armor.clone();
                player_with_weapon_and_armor_and_1_ring.damage += first_ring.damage;
                player_with_weapon_and_armor_and_1_ring.armor += first_ring.armor;
                new_cost += first_ring.cost;
                // Player can choose only 1 ring
                layouts.push((player_with_weapon_and_armor_and_1_ring.clone(), new_cost));
                for second_ring in shop.rings.iter() {
                    // Finally, player chooses 2nd ring
                    // Which must be different from first ring
                    if second_ring.cost == first_ring.cost {
                        continue;
                    }
                    let mut player_with_weapon_and_armor_and_2_rings =
                        player_with_weapon_and_armor_and_1_ring.clone();
                    player_with_weapon_and_armor_and_2_rings.damage += second_ring.damage;
                    player_with_weapon_and_armor_and_2_rings.armor += second_ring.armor;
                    new_cost += second_ring.cost;
                    layouts.push((player_with_weapon_and_armor_and_2_rings, new_cost));
                    new_cost -= second_ring.cost;
                }
                new_cost -= first_ring.cost;
            }
        }
        // Player can also choose 1 or 2 rings without armor
        for first_ring in shop.rings.iter() {
            // Player now chooses 1 first ring
            let mut player_with_weapon_and_1_ring = player_with_weapon.clone();
            player_with_weapon_and_1_ring.damage += first_ring.damage;
            player_with_weapon_and_1_ring.armor += first_ring.armor;
            new_cost += first_ring.cost;
            // Player can choose only 1 ring
            layouts.push((player_with_weapon_and_1_ring.clone(), new_cost));
            for second_ring in shop.rings.iter() {
                // Finally, player chooses 2nd ring
                // Which must be different from first ring
                if second_ring.cost == first_ring.cost {
                    continue;
                }
                let mut player_with_weapon_and_2_rings = player_with_weapon_and_1_ring.clone();
                player_with_weapon_and_2_rings.damage += second_ring.damage;
                player_with_weapon_and_2_rings.armor += second_ring.armor;
                new_cost += second_ring.cost;
                layouts.push((player_with_weapon_and_2_rings, new_cost));
                new_cost -= second_ring.cost;
            }
            new_cost -= first_ring.cost;
        }
    }
    let mut min_cost_win = i32::MAX;
    for (player_test, cost) in layouts.iter() {
        if *cost < min_cost_win && play(boss.clone(), player_test.clone()) {
            min_cost_win = *cost;
        }
    }
    min_cost_win
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
    assert_eq!(res, 111);
    println!("> DAY21 - part 1: OK!");
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
