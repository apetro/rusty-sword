use rand::seq::SliceRandom;
use rand::Rng;
use std::io;

fn main() {
    println!("What is your character's name?");

    let mut character_name = String::new();
    let mut inventory = Inventory {
        lesser_spirit_coins: 0,
    };

    io::stdin()
        .read_line(&mut character_name)
        .expect("Failed to read line");

    let character_name = character_name.trim();

    let mut player_character = PlayerCharacter {
        name: character_name.to_string(),
        health: 10,
        evade: 10,
    };

    println!("Greetings, {}.", player_character.name);

    let nest_of_spiders = Dungeon { name: "nest of spiders".to_string(), flavor_text: "A dark nest of scary spiders. It's a tough job clearing them out, but someone has got to do it.".to_string(), monsters: spiders()};

    let dungeon_result = explore_dungeon(player_character, nest_of_spiders, inventory);

    player_character = dungeon_result.player_character;
    inventory = dungeon_result.inventory;

    println!(
        "{} ended the game with {} lesser spirit coins.",
        player_character.name,
        inventory.lesser_spirit_coins
    );

    println!("The end.");
}

fn roll_die(dice_size: usize) -> usize {
    rand::thread_rng().gen_range(1..=dice_size)
}

fn d12() -> usize {
    roll_die(12)
}

fn d20() -> usize {
    roll_die(20)
}

struct PlayerCharacter {
    name: String,
    health: usize,
    evade: usize,
}

struct Monster {
    name: String,          // giant spider
    attack_verb: String,   // bites ("The giant spider bites you.")
    hit_difficulty: usize, // 10
}

struct CombatResult {
    player_character: PlayerCharacter,
    lesser_spirit_coins: usize,
}

fn combat_with_monster(mut player_character: PlayerCharacter, monster: &Monster) -> CombatResult {
    println!("You have encountered a {}.", monster.name);

    'monster: loop {
        if d20() >= player_character.evade {
            player_character.health = player_character.health - 1;

            println!(
                "The {} {} you. You have {} health remaining.",
                monster.name, monster.attack_verb, player_character.health
            );

            if player_character.health < 1 {
                println!("You have succumbed to your wounds.");
                break 'monster;
            }
        }

        if d12() + d12() >= monster.hit_difficulty {
            println!("You have defeated the {}.", monster.name);

            println!(
                "The {} evaporates in a cloud of foul-smelling rainbow smoke.",
                monster.name
            );
            println!("When the smoke dissipates, a lesser spirit coin remains, which you grab.");
            println!();
            break 'monster;
        } else {
            println!(
                "You swing your rusty sword at the {}, but miss.",
                monster.name
            )
        }
    }

    CombatResult {
        player_character,
        lesser_spirit_coins: 1,
    }
}

struct Dungeon {
    name: String,        // nest of spiders
    flavor_text: String, // "A nest of scary spiders. It's a tough job clearing them out, but someone has got to do it."
    monsters: Vec<Monster>,
}

fn spiders() -> Vec<Monster> {
    vec![
        Monster {
            name: "big spider".to_string(),
            attack_verb: "bites".to_string(),
            hit_difficulty: 7,
        },
        Monster {
            name: "large spider".to_string(),
            attack_verb: "bites".to_string(),
            hit_difficulty: 8,
        },
        Monster {
            name: "extra large spider".to_string(),
            attack_verb: "bites".to_string(),
            hit_difficulty: 9,
        },
        Monster {
            name: "giant spider".to_string(),
            attack_verb: "bites".to_string(),
            hit_difficulty: 10,
        },
        Monster {
            name: "huge spider".to_string(),
            attack_verb: "chomps on".to_string(),
            hit_difficulty: 11,
        },
        Monster {
            name: "monstrous spider".to_string(),
            attack_verb: "spits venom at".to_string(),
            hit_difficulty: 12,
        },
        Monster {
            name: "eltritch spider".to_string(),
            attack_verb: "curses".to_string(),
            hit_difficulty: 13,
        },
        Monster {
            name: "brood mother".to_string(),
            attack_verb: "bites".to_string(),
            hit_difficulty: 18,
        },
        Monster {
            name: "shield spider".to_string(),
            attack_verb: "crushes".to_string(),
            hit_difficulty: 10,
        },
        Monster {
            name: "frost spider".to_string(),
            attack_verb: "freezes".to_string(),
            hit_difficulty: 10,
        },
    ]
}

struct DungeonResult {
    player_character: PlayerCharacter,
    inventory: Inventory,
}

fn explore_dungeon(
    mut player_character: PlayerCharacter,
    dungeon: Dungeon,
    mut inventory: Inventory,
) -> DungeonResult {
    println!("{}", dungeon.flavor_text);

    'main: loop {
        let mut action = String::new();
        println!();
        println!(
            "Would you like to HUNT a monster, LEAVE {}, or view your INVENTORY?",
            dungeon.name
        );
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");
        let action = action.trim();

        match action {
            "HUNT" => {
                let monster = dungeon_denizen(&dungeon);
                let combat_result = combat_with_monster(player_character, monster);
                player_character = combat_result.player_character;
                if player_character.health < 1 {
                    break 'main;
                }
                inventory.lesser_spirit_coins =
                    inventory.lesser_spirit_coins + combat_result.lesser_spirit_coins;
            }

            "LEAVE" => {
                println!("Discretion is the better part of valor. You leave the nest.");
                break 'main;
            }

            "INVENTORY" => {
                println!("You are wielding a rusty sword in your dominant hand.");
                println!(
                    "You have {} lesser spirit coins.",
                    inventory.lesser_spirit_coins
                );
            }

            _ => println!("I don't know what how to {action}."),
        }
    }

    DungeonResult {
        player_character,
        inventory,
    }
}

fn dungeon_denizen(dungeon: &Dungeon) -> &Monster {
    let mut rng = rand::thread_rng();
    dungeon
        .monsters
        .choose(&mut rng)
        .expect("Expected the dungeon to have Monsters in it to choose from.")
}

struct Inventory {
    lesser_spirit_coins: usize,
}
