use rand::{thread_rng, Rng};

pub fn gen_random() -> u8{
    let mut rng = thread_rng();

    rng.gen_range(0..100)
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn generate_try_base_on_level(level: &u32) -> u3 {
    let tries: i32 = (11 - level) as i32;

    if tries < 1 {
        return 3;
    }

    return tries;
}

pub fn generate_try_tracker(level: &u32, tries: Vec<bool>){

}