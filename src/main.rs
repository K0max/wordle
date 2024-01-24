mod wordle;
mod letter;

// use std::env;
use wordle::Wordle;
use wordle::GameState;

fn main() {
    let mut wordle_game = Wordle::new();
    println!("Welcome to Wordle! Answer is {:?}", wordle_game.answer);
    loop {
        let mut guess = String::new();
        println!("Please input your guess: ");
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim().to_uppercase();
        wordle_game.update_round(&guess);
        if wordle_game.game_state != GameState::Playing {
            println!("If you want to play again, please input y, if not, please input anything else");
            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).expect("Failed to read line");
            match choice.trim() {
                "y" => {
                    wordle_game = Wordle::new();
                    println!("Welcome to Wordle! Answer is {:?}", wordle_game.answer);
                }
                _ => break,
            }
        }
    }
}