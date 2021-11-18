use std::io;
use ansi_term::Colour;

mod util;

fn main() {

    println!("{}", Colour::Cyan.bold().paint("Welcome to Number Guess!"));

    loop {
        let random_number: u8 = util::gen_random();
        let mut input = String::new();

        println!("{} {}",
            Colour::Yellow.paint("Guess A Number"),
            Colour::White.paint("vvv")
        );

        io::stdin()
            .read_line(&mut input)
            .expect("hmm i cant get input");

        let input = input.trim();

        println!("you guess {} i guessed {}", input, random_number)
    }
}

