use rand::{thread_rng, Rng};

pub fn gen_random() -> u8{
    let mut rng = thread_rng();

    rng.gen_range(0..100)
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}