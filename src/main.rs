use std::io;
use ansi_term::Colour;

mod util;

fn main() {

    util::clear_terminal();

    println!("{}", Colour::Cyan.bold().paint("Welcome to Number Guess Game!"));

    println!("{}", Colour::White.paint("-------------------------------"));

    let mut random_number: u8 = util::gen_random();
    let mut level: u32 = 1;
    let mut try_tracker: Vec<bool> = vec![];

    loop {
        let mut input = String::new();

        util::print_level(&level);
        util::print_try_count(&try_tracker);
        util::generate_try_tracker(&level, &try_tracker);

        util::print_guess_and_type();

        io::stdin()
            .read_line(&mut input)
            .expect("hmm i cant get input");

        let input: u8 = input.trim().parse().expect("not a string or number");

        if input == random_number {
            level += 1;
            try_tracker = vec![];
            random_number = util::gen_random();
        } else {
            try_tracker.push(false);
        }
    }


    util::clear_terminal();
}

