use rand::Rng;

fn roll_die(dice_size: usize) -> usize {
    rand::thread_rng().gen_range(1..=dice_size)
}

pub fn d12() -> usize {
    roll_die(12)
}

pub fn d20() -> usize {
    roll_die(20)
}
