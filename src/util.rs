use rand::{thread_rng, Rng};
use ansi_term::Colour;

pub fn gen_random() -> u8{
    let mut rng = thread_rng();

    rng.gen_range(0..100)
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn print_guess_and_type(){
    println!("{} {}",
             Colour::Yellow.paint("Guess And Type Your Number"),
             Colour::White.paint("Range 0 <--> 100")
    );
}

pub fn print_level(level: &u32){
    print!("{} {} | ",
           Colour::Yellow.paint("Level >> "),
           Colour::Yellow.paint(level.to_string())
    );
}

pub fn print_try_count(try_tracker: &Vec<bool>){
    print!("{} {} | ",
           Colour::Purple.paint("Try >> "),
           Colour::Purple.paint(try_tracker.len().to_string())
    );
}

pub fn generate_try_base_on_level(level: &u32) -> u32 {
    let tries: i32 = (11 - level) as i32;

    if tries < 1 {
        return 3;
    }

    return tries as u32;
}

pub fn generate_try_tracker(level: &u32, tries: &Vec<bool>){

    print!("{}", Colour::Blue.paint("Your Tries >> "));

    for index in 1..generate_try_base_on_level(level) {
        print!("{}", Colour::White.paint("-"));
    }

    println!(" ")
}