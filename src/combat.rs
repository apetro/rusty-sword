
use crate::PlayerCharacter;
use crate::monster::Monster;
use crate::dice::d12;
use crate::dice::d20;

pub struct CombatResult {
    pub player_character: PlayerCharacter,
    pub lesser_spirit_coins: usize,
}

pub fn combat_with_monster(mut player_character: PlayerCharacter, monster: &Monster) -> CombatResult {
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
