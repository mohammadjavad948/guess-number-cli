use rand::{thread_rng, Rng};
use ansi_term::Colour;
use std::io;

pub struct Game {
    random_number: u8,
    level: usize,
    try_tracker: Vec<bool>,
    user_input: String
}

impl Game {

    pub fn new() -> Game {
        Game {
            try_tracker: vec![],
            level: 1,
            random_number: Game::generate_random_number(),
            user_input: String::new()
        }
    }

    pub fn is_not_playable(&self) -> bool {
        self.try_tracker.len() == self.generate_try_base_on_level()
    }

    pub fn get_user_input(&mut self){
        io::stdin()
            .read_line(&mut self.user_input)
            .expect("hmm i cant get input");
    }

    pub fn calculate_user_input(&mut self){
        let input: u8 = self.user_input.trim().parse().expect("not a string or number");

        if input == self.random_number {
            self.level += 1;
            self.try_tracker = vec![];
            self.random_number = Game::generate_random_number();
        } else {
            self.try_tracker.push(false);
        }
    }


    pub fn print_guess_and_type(&self){
        println!("{} {}",
                 Colour::Yellow.paint("Guess And Type Your Number"),
                 Colour::White.paint("Range 0 <--> 10")
        );
    }

    pub fn print_level(&self){
        print!("{} {} | ",
               Colour::Yellow.paint("Level >> "),
               Colour::Yellow.paint(self.level.to_string())
        );
    }

    pub fn print_try_count(&self){
        print!("{} {} | ",
               Colour::Purple.paint("Try >> "),
               Colour::Purple.paint(self.try_tracker.len().to_string())
        );
    }

    pub fn generate_try_base_on_level(&self) -> usize {
        let tries: isize = (11 - self.level) as isize;

        if tries < 1 {
            return 3;
        }

        return tries as usize;
    }

    pub fn generate_try_tracker(&self){

        print!("{}", Colour::Blue.paint("Your Tries >> "));

        let tries: Vec<usize> = vec![0; Game::generate_try_base_on_level(&self)];
        let fails = self.try_tracker.len();

        tries
            .iter()
            .map(|x| {
                if x <= &fails {
                    print!("{}", Colour::Red.paint("-"))
                } else {
                    print!("{}", Colour::White.paint("-"))
                }
            });
    }


    pub fn generate_random_number() -> u8 {
        let mut rng = thread_rng();

        rng.gen_range(0..10)
    }

    pub fn print_info(&self){
        self.print_level();
        self.print_try_count();
        self.generate_try_tracker();

        println!(" ");

        self.print_guess_and_type()
    }

    pub fn clear_terminal() {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn end_game(&self){
        println!("{} {} {} {}",
            Colour::Red.paint("You Lost At Level :"),
            Colour::Red.paint(self.level.to_string()),
            Colour::Red.paint("And Try :"),
            Colour::Red.paint(self.try_tracker.len().to_string())
        )
    }
}