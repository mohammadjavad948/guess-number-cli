use rand::{thread_rng, Rng};
use ansi_term::Colour;
use std::time::Duration;
use std::io;

pub fn gen_random() -> u8{
    let mut rng = thread_rng();

    rng.gen_range(0..10)
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn print_guess_and_type(){
    println!("{} {}",
             Colour::Yellow.paint("Guess And Type Your Number"),
             Colour::White.paint("Range 0 <--> 10")
    );
}

pub fn print_response_time(time: &Duration){
    print!("{} {:.2?}",
           Colour::Green.paint("Response Time >> "),
           time
    );
}

pub fn print_level(level: &usize){
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

pub fn generate_try_base_on_level(level: &usize) -> usize {
    let tries: isize = (11 - level) as isize;

    if tries < 1 {
        return 3;
    }

    return tries as usize;
}

pub fn generate_try_tracker(level: &usize, tries: &Vec<bool>){

    print!("{}", Colour::Blue.paint("Your Tries >> "));

    for i in 0..generate_try_base_on_level(level) {

        let state = tries.get(i);

        match state {
            Some(_) => {
                print!("{}", Colour::Red.paint("-"));
            },
            None => {
                print!("{}", Colour::White.paint("-"));
            }
        }
    }

    print!(" | ")
}

pub fn end_game(level: &usize, try_tracker: &Vec<bool>){

    clear_terminal();

    println!("{} {} {} {}",
             Colour::Red.paint("You Lost At Level :"),
             Colour::Red.paint(level.to_string()),
             Colour::Red.paint("And Try :"),
             Colour::Red.paint(try_tracker.len().to_string())
    )
}

pub struct Game {
    random_number: u8,
    level: usize,
    try_tracker: Vec<bool>,
    user_input: String
}

impl Game {

    fn new() -> Game {
        Game {
            try_tracker: vec![],
            level: 1,
            random_number: Game::generate_random_number(),
            user_input: String::new()
        }
    }

    fn get_user_input(&mut self){
        io::stdin()
            .read_line(&mut self.user_input)
            .expect("hmm i cant get input");
    }

    fn calculate_user_input(&mut self){
        let input: u8 = self.user_input.trim().parse().expect("not a string or number");

        if input == self.random_number {
            self.level += 1;
            self.try_tracker = vec![];
            self.random_number = Game::generate_random_number();
        } else {
            self.try_tracker.push(false);
        }
    }

    fn generate_random_number() -> u8 {
        let mut rng = thread_rng();

        rng.gen_range(0..10)
    }

    fn clear_terminal() {
        print!("\x1B[2J\x1B[1;1H");
    }
}