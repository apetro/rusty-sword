pub struct Inventory {
    pub lesser_spirit_coins: usize,
}

pub fn print_inventory(inventory: &Inventory) {
    println!("You are wielding a rusty sword in your dominant hand.");
    println!(
        "You have {} lesser spirit coins.",
        inventory.lesser_spirit_coins
    );
}
