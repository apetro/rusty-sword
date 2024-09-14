use std::io;

fn main() {
    println!("What is your character's name?");

    let mut character_name = String::new();
    let mut character_health = 10;

    io::stdin()
        .read_line(&mut character_name)
        .expect("Failed to read line");

    let character_name = character_name.trim();

    println!("Greetings, {}.", character_name);

    println!("You are in a dark, dusty dungeon.");

    loop {
        let mut action = String::new();
        println!("You have encountered a giant spider.");
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");
        // let action = action.trim();
        character_health = character_health - 1;

        println!("The spider bites you. You have {character_health} health remaining.");

        if character_health < 1 {
            println!("You have succumbed to your wounds. The end.");
            break;
        }

        println!("You have defeated the giant spider.");
        println!();
    }
}
