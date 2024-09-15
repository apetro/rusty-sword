use std::io;

pub mod dice;
pub mod inventory;
pub mod dungeon;
pub mod monster;

use crate::dungeon::explore_dungeon;
use crate::dungeon::Dungeon;

use crate::dice::d12;
use crate::dice::d20;

use crate::inventory::print_inventory;
use crate::inventory::Inventory;

use crate::monster::spiders;
use crate::monster::Monster;

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

    loop {
        let mut action = String::new();
        println!();
        println!(
            "Would you like to clear out the SPIDERS nest, confront the BRIDGE troll, view your INVENTORY, or RETIRE from adventuring?"
        );
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");
        let action = action.trim();

        match action {
            "SPIDERS" => {
                let nest_of_spiders = Dungeon { name: "nest of spiders".to_string(), flavor_text: "A dark nest of scary spiders. It's a tough job clearing them out, but someone has got to do it.".to_string(), monsters: spiders()};

                let dungeon_result = explore_dungeon(player_character, nest_of_spiders, inventory);

                player_character = dungeon_result.player_character;
                inventory = dungeon_result.inventory;
            }
            "BRIDGE" => {
                let troll_bridge: Dungeon = Dungeon {
                    name: "troll bridge".to_string(),
                    flavor_text: "A bridge under which lives a grumpy troll".to_string(),
                    monsters: vec![Monster {
                        name: "Bridge Troll".to_string(),
                        attack_verb: "whomps".to_string(),
                        hit_difficulty: 20,
                    }],
                };
                let dungeon_result = explore_dungeon(player_character, troll_bridge, inventory);

                player_character = dungeon_result.player_character;
                inventory = dungeon_result.inventory;
            }

            "INVENTORY" => print_inventory(&inventory),

            "RETIRE" => break,

            _ => println!("{} wasn't a choice?", action),
        }

        if player_character.health < 1 {
            break;
        }
    }

    println!(
        "{} ended the game with {} lesser spirit coins.",
        player_character.name, inventory.lesser_spirit_coins
    );

    println!("The end.");
}

struct PlayerCharacter {
    name: String,
    health: usize,
    evade: usize,
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

