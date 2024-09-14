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

    let mut player_character = PlayerCharacter {
        name: character_name.to_string(),
        health: 10,
        evade: 10,
    };

    println!("Greetings, {}.", player_character.name);

    println!("You are in a dark nest of scary spiders. It's a tough job clearing them out, but someone has got to do it.");

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


                let spider = level_one_spider();

                println!("You have encountered a {}.", spider.name);

                'spider: loop {
                    if d20() >= player_character.evade {
                        player_character.health = player_character.health - 1;

                        println!(
                            "The {} {} you. You have {} health remaining.", spider.name, spider.attack_verb, player_character.health
                        );

                        if player_character.health < 1 {
                            println!("You have succumbed to your wounds.");
                            break 'main;
                        }
                    }

                    if d12() + d12() >= spider.hit_difficulty {
                        println!("You have defeated the {}.", spider.name);

                        println!(
                            "The {} evaporates in a cloud of foul-smelling rainbow smoke.", spider.name
                        );
                        println!("Incredibly, when the smoke clears it reveals an iron spirit coin where once there was a monster.");
                        character_treasure = character_treasure + 1;
                        println!(
                        "You take the coin. You now have {character_treasure} lesser spirit coins."
                    );
                        println!();
                        break 'spider;
                    } else {
                        println!("You swing your rusty sword at the {}, but miss.", spider.name)
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

struct Monster {
    name: String,          // giant spider
    attack_verb: String,   // bites ("The giant spider bites you.")
    hit_difficulty: usize, // 10
}

fn level_one_spider() -> Monster {
    let spider_type = roll_die(10);

  match spider_type {
    1 => Monster {name: "big spider".to_string(), attack_verb: "bites".to_string(), hit_difficulty: 7},
    2 => Monster {name: "large spider".to_string(), attack_verb: "bites".to_string(), hit_difficulty: 8},
    3 => Monster {name: "extra large spider".to_string(), attack_verb: "bites".to_string(), hit_difficulty: 9},
    4 => Monster {name: "giant spider".to_string(), attack_verb: "bites".to_string(), hit_difficulty: 10},
    5 => Monster {name: "huge spider".to_string(), attack_verb: "chomps on".to_string(), hit_difficulty: 11},
    6 => Monster {name: "monstrous spider".to_string(), attack_verb: "spits venom at".to_string(), hit_difficulty: 12},
    7 => Monster {name: "eltritch spider".to_string(), attack_verb: "curses".to_string(), hit_difficulty: 13},
    8 => Monster {name: "brood mother".to_string(), attack_verb: "bites".to_string(), hit_difficulty: 18},
    9 => Monster {name: "shield spider".to_string(), attack_verb: "crushes".to_string(), hit_difficulty: 10},
    10 => Monster {name: "frost spider".to_string(), attack_verb: "freezes".to_string(), hit_difficulty: 10},
    _ => Monster {name: "generic spider".to_string(), attack_verb: "generically attacks".to_string(), hit_difficulty: 10},
  }
}
