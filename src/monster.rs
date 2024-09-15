pub struct Monster {
    pub name: String,          // giant spider
    pub attack_verb: String,   // bites ("The giant spider bites you.")
    pub hit_difficulty: usize, // 10
}

pub fn spiders() -> Vec<Monster> {
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
