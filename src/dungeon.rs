
use crate::combat::combat_with_monster;
use crate::print_inventory;
use crate::Inventory;
use crate::Monster;
use crate::PlayerCharacter;
use rand::prelude::SliceRandom;
use std::io;

pub struct Dungeon {
    pub name: String,        // nest of spiders
    pub flavor_text: String, // "A nest of scary spiders. It's a tough job clearing them out, but someone has got to do it."
    pub monsters: Vec<Monster>,
}

pub struct DungeonResult {
    pub player_character: PlayerCharacter,
    pub inventory: Inventory,
}

pub fn explore_dungeon(
    mut player_character: PlayerCharacter,
    dungeon: Dungeon,
    mut inventory: Inventory,
) -> DungeonResult {
    println!("{}", dungeon.flavor_text);

    'main: loop {
        let mut action = String::new();
        println!();
        println!("Location: {}", dungeon.name);
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

            "INVENTORY" => print_inventory(&inventory),

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
