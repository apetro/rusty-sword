use rand::Rng;
use std::io;

fn main() {
    println!("What is your character's name?");

    let mut character_name = String::new();
    let mut character_treasure = 0;


    io::stdin()
        .read_line(&mut character_name)
        .expect("Failed to read line");

    let character_name = character_name.trim();

    let mut player_character = PlayerCharacter { name: character_name.to_string(), health: 10, evade: 10 };

    println!("Greetings, {}.", player_character.name);

    println!("You are in a dark nest of giant spiders. It's a tough job clearing them out, but someone has got to do it.");

    'main: loop {
        let mut action = String::new();
        println!();
        println!("Would you like to HUNT a spider or LEAVE the nest?");
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");
        let action = action.trim();

        match action {
            "HUNT" => {
                println!("You have encountered a giant spider.");

                'spider: loop {
                    if d20() >= player_character.evade {
                        player_character.health = player_character.health - 1;

                        println!(
                            "The spider bites you. You have {} health remaining.", player_character.health
                        );

                        if player_character.health < 1 {
                            println!("You have succumbed to your wounds.");
                            break 'main;
                        }
                    }

                    if d12() + d12() >= 10 {
                        println!("You have defeated the giant spider.");

                        println!(
                            "The spider evaporates in a cloud of foul-smelling rainbow smoke."
                        );
                        println!("Incredibly, when the smoke clears it reveals an iron spirit coin where once there was a monster.");
                        character_treasure = character_treasure + 1;
                        println!(
                        "You take the coin. You now have {character_treasure} lesser spirit coins."
                    );
                        println!();
                        break 'spider;
                    } else {
                        println!("You swing your rusty sword at the spider, but miss.")
                    }
                }
            },

            "LEAVE" => {
              println!("Discretion is the better part of valor. You leave the nest.");
              break 'main;
            },

            _ => println!("I don't know what that is."),
        }
    }

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

