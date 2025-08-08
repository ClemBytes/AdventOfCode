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

#[derive(PartialEq, Clone, Copy, Debug)]
enum Actions {
    // Boss only has one action: attack
    Attack,
    // Player has 5 different spells
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Actions {
    fn mana(&self) -> i32 {
        match self {
            Actions::Attack => 0, // boss action
            Actions::MagicMissile => 53,
            Actions::Drain => 73,
            Actions::Shield => 113,
            Actions::Poison => 173,
            Actions::Recharge => 229,
        }
    }
}

fn parse(raw_input: &str) -> Boss {
    let (hit_points, damage) = raw_input.split_once("\n").unwrap();
    Boss {
        hit_points: hit_points.trim().split(": ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap(),
        damage: damage.trim().split(": ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap(),
        poison_effect: 0,
    }
}

fn apply_effects(boss: &mut Boss, player: &mut Player) {
    // Poison deals 3 damages to the boss every turn for 6 turns
    if boss.poison_effect > 0 {
        boss.hit_points -= 3;
        boss.poison_effect -= 1;
    }

    // Shield increases player's armor by 7 during 6 turns
    if player.shield_effect > 0 {
        player.armor = 7;
        player.shield_effect -= 1;
    } else {
        player.armor = 0;
    }

    // Recharge increases player's mana by 101 every turn for 5 turns
    if player.recharge_effect > 0 {
        player.mana += 101;
        player.recharge_effect -= 1;
    }
}

fn play_turn(boss: &mut Boss, player: &mut Player, action: Actions) {
    apply_effects(boss, player);
    match action {
        Actions::Attack => {
            player.hit_points -= (boss.damage - player.armor).max(1);
        }
        Actions::MagicMissile => {
            boss.hit_points -= 4;
        }
        Actions::Drain => {
            boss.hit_points -= 2;
            player.hit_points += 2;
        }
        Actions::Shield => {
            if player.shield_effect > 0 {
                panic!(
                    "Player can't cast Shield when its effect is still active (for {} turns)!",
                    player.shield_effect
                );
            }
            player.shield_effect = 6;
        }
        Actions::Poison => {
            if boss.poison_effect > 0 {
                panic!(
                    "Player can't cast Poison when its effect is still active (for {} turns)!",
                    boss.poison_effect
                );
            }
            boss.poison_effect = 6;
        }
        Actions::Recharge => {
            if player.recharge_effect > 0 {
                panic!(
                    "Player can't cast Recharge when its effect is still active (for {} turns)!",
                    player.recharge_effect
                );
            }
            player.recharge_effect = 5;
        }
    }
}

fn recursive_find_cheapest_mana_win(
    boss: Boss,
    player: Player,
    is_player_turn: bool,
    mana_used: i32,
    list_of_actions: &mut Vec<Actions>,
) -> i32 {
    // This function returns how much mana was used during the game (and i32::MAX if lose)
    // Check if already win or lose
    if boss.hit_points <= 0 {
        // WIN!
        // println!("WIN: mana used: {mana_used}\n{list_of_actions:?}\n");
        return mana_used;
    } else if player.hit_points <= 0 {
        // Lose…
        if mana_used < 1415 && mana_used > 932 {
            // println!("Lose: {mana_used}\n{player:?}\n{boss:?}\n{list_of_actions:?}\n");
        }
        return i32::MAX;
    }


    // Else
    if is_player_turn {
        let mut min_mana_win = i32::MAX;
        let player_actions = vec![
            Actions::MagicMissile,
            Actions::Drain,
            Actions::Shield,
            Actions::Poison,
            Actions::Recharge,
        ];
        for action in player_actions {
            // Can't do an action with effect if effect already active
            if action == Actions::Shield && player.shield_effect > 0 {
                continue;
            }
            if action == Actions::Poison && boss.poison_effect > 0 {
                continue;
            }
            if action == Actions::Recharge && player.recharge_effect > 0 {
                continue;
            }

            let action_mana = action.mana();
            // Can't do an action if not enough mana
            if player.mana < action_mana {
                continue;
            }

            let mut boss_clone = boss.clone();
            let mut player_clone = player.clone();
            player_clone.mana -= action_mana;
            play_turn(&mut boss_clone, &mut player_clone, action);

            list_of_actions.push(action);

            let res_mana =
                recursive_find_cheapest_mana_win(boss_clone, player_clone, false, mana_used + action_mana, list_of_actions);

            list_of_actions.pop();

            if res_mana < min_mana_win {
                min_mana_win = res_mana;
            }
        }
        min_mana_win
    } else {
        let mut boss_clone = boss.clone();
        let mut player_clone = player.clone();
        play_turn(&mut boss_clone, &mut player_clone, Actions::Attack);
        recursive_find_cheapest_mana_win(boss_clone, player_clone, true, mana_used, list_of_actions)
    }
}

fn day22_part1(boss: Boss, player: Player) {
    // Exemple tests
    let mut boss_example = boss.clone();
    boss_example.hit_points = 13;
    boss_example.damage = 8;
    let mut player_example = player.clone();
    player_example.hit_points = 10;
    player_example.mana = 250;
    let res = recursive_find_cheapest_mana_win(boss_example, player_example, true, 0, &mut vec![]);
    println!("Result example 1 part 1: {res}");

    let mut boss_example = boss.clone();
    boss_example.hit_points = 14;
    boss_example.damage = 8;
    let mut player_example = player.clone();
    player_example.hit_points = 10;
    player_example.mana = 250;
    let res = recursive_find_cheapest_mana_win(boss_example, player_example, true, 0, &mut vec![]);
    println!("Result example 2 part 1: {res}");

    // Solve puzzle
    let res = recursive_find_cheapest_mana_win(boss, player, true, 0, &mut vec![]);
    println!("Result part 1: {res}");
    // assert_eq!(res, ); // 932 is too low, 1415 is too high
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
