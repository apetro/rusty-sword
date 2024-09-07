use std::io;
use rand::Rng;


fn main() {
    println!("What is your character's name?");

    let mut character_name = String::new();

    io::stdin()
        .read_line(&mut character_name)
        .expect("Failed to read line");

    let character_name = character_name.trim();

    println!("Greetings, {}.", character_name);

    let d_20 = rand::thread_rng().gen_range(1..=20);

    println!("You rolled {d_20}.");
}

