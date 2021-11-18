use std::io;
use ansi_term::Colour;

mod util;

fn main() {

    util::clear_terminal();

    println!("{}", Colour::Cyan.bold().paint("Welcome to Number Guess Game!"));

    println!("{}", Colour::White.paint("-------------------------------"));

    'game_loop: loop {
        let random_number: u8 = util::gen_random();
        let mut level: u32 = 1;
        let mut try_tracker: Vec<bool> = vec![];

        'try_loop: loop {
            let mut input = String::new();

            util::print_level(&level);
            util::print_try_count(&try_tracker);

            util::generate_try_tracker(&level, &try_tracker);

            println!("{} {}",
                     Colour::Yellow.paint("Guess And Type Your Number"),
                     Colour::White.paint("Range 0 <--> 100")
            );

            io::stdin()
                .read_line(&mut input)
                .expect("hmm i cant get input");

            let input: u8 = input.trim().parse().expect("not a string");
        }


        util::clear_terminal();
    }
}

