mod util;

use ansi_term::Colour;
use util::Game;

fn main() {

    Game::clear_terminal();

    println!("{}", Colour::Cyan.bold().paint("Welcome to Number Guess Game!"));

    println!("{}", Colour::White.paint("-------------------------------"));

    let mut game = Game::new();

    loop {

        if game.is_not_playable() {
            break;
        }

        game.print_info();

        game.get_user_input();

        game.calculate_user_input();

        Game::clear_terminal();
    }

    game.end_game();
}

