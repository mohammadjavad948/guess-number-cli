use std::io;

mod util;

fn main() {

    loop {
        let x: u8 = util::gen_random();
        let mut input = String::new();

        println!("random number generated !");

        println!("gimme Input :");

        io::stdin()
            .read_line(&mut input)
            .expect("hmm i cant get input");

        let input = input.trim();

        println!("you guess {} i guessed {}", input, x)
    }
}

