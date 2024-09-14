use rand::Rng;
use std::io;

fn main() {
    println!("What is your character's name?");

    let mut character_name = String::new();
    let mut character_health = 10;
    let mut character_treasure = 0;
    let character_evade = 10;

    io::stdin()
        .read_line(&mut character_name)
        .expect("Failed to read line");

    let character_name = character_name.trim();

    println!("Greetings, {}.", character_name);

    println!("You are in a dark, dusty dungeon.");

    'main: loop {
        let mut action = String::new();
        println!("You have encountered a giant spider.");

        'spider: loop {
            io::stdin()
                .read_line(&mut action)
                .expect("Failed to read line");
            // let action = action.trim();

            if d20() >= character_evade {
                character_health = character_health - 1;

                println!("The spider bites you. You have {character_health} health remaining.");

                if character_health < 1 {
                    println!("You have succumbed to your wounds.");
                    break 'main;
                }
            }

            if d12() + d12() >= 10 {
                println!("You have defeated the giant spider.");

                println!("The spider evaporates in a cloud of foul-smelling rainbow smoke.");
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
