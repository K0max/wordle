use crate::letter::{Letter, LetterState};
use std::fs::File;
use std::io::Read; // read_to_string
use rand::Rng;
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum GameState {
    Win,
    Lose,
    Playing,
}

const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G',
    'H', 'I', 'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

pub struct Wordle {
    pub answer: String,
    pub guessed_words: Vec<String>,
    pub used_letters: Vec<Letter>,
    // pub used_letters: HashMap<String, LetterState>,
    pub round: u32,
    pub game_state: GameState,
}

impl Wordle {
    // Initialize a new Wordle game
    pub fn new() -> Wordle {
        Wordle {
            answer: String::from(Wordle::generate_answer()),
            guessed_words: Vec::new(),
            used_letters: Vec::new(),
            round: 0,
            game_state: GameState::Playing,
        }
    }
    pub fn generate_answer() -> String {
        let file_path = String::from("assets/words.txt");
        let mut words_file = File::open(&file_path).expect("Failed to open words.txt");
        let mut contents = String::new();
        words_file.read_to_string(&mut contents).expect(&format!("Failed to read {}", &file_path));
        let words: Vec<&str> = contents.split("\n").collect();
        let words = words.iter().map(|word| word.to_uppercase()).collect::<Vec<String>>();
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..words.len());
        // remember to trim the escape character \n
        words[random_index].trim().to_string()
    }
    pub fn update_round(&mut self, guess: &str) {
        self.round += 1;

        for guessed_word in &self.guessed_words {
            if guess == guessed_word {
                println!("You already guessed {}, please input another word:\n", guess);
                break;
                // input guess
                // let mut guess = String::new();
                // std::io::stdin().read_line(&mut guess).expect("Failed to read line");
            }
        }

        self.guessed_words.push(guess.to_string());

        let mut guess_letters: Vec<Letter> = Vec::new();
        for i in 0..guess.len() {
            guess_letters.push(Letter::new(guess.chars().nth(i).unwrap(), LetterState::Wrong));
            for j in 0..self.answer.len() {
                if guess_letters[i].state != LetterState::Wrong {
                    break;
                }
                if guess.chars().nth(i).unwrap() == self.answer.chars().nth(j).unwrap() {
                    if i == j {
                        guess_letters[i].state = LetterState::Correct;
                    }
                    else {
                        guess_letters[i].state = LetterState::WrongSpot;
                    }
                }
            }
        }

        for guess_letter in guess_letters {
            let mut found = false;
            for used_letter in &mut self.used_letters {
                if guess_letter == *used_letter {
                    found = true;
                    break;
                }
            }
            if !found {
                self.used_letters.push(guess_letter);
            }
        }

        // clear console
        print!("\x1b[2J");
        print!("\x1b[H");
        self.print_wordle();
        println!("");

        if guess == self.answer {
            self.game_state = GameState::Win;
            self.win_cg();
        } else if self.round == 6 {
            self.game_state = GameState::Lose;
            self.defeated_cg();
        }
    }

    pub fn win_cg(&mut self) {
        println!("You won! The word was {}", self.answer);
    }

    pub fn defeated_cg(&mut self) {
        println!("You lost! The word was {}", self.answer);
    }

    // print alphabet, used letters will be in specific color
    pub fn print_alphabet(&self) {
        let mut used_alph = Vec::new();
        let mut used_alph_hash = HashMap::new();
        for letter in &self.used_letters {
            used_alph.push(letter.alph);
            used_alph_hash.insert(letter.alph, &letter.state);
        }
        for alph in ALPHABET.iter() {
            match used_alph_hash.get(alph) {
                Some(LetterState::Correct) => {
                    print!("{} ", console::style(format!("{}", alph)).bold().green());
                }
                Some(LetterState::WrongSpot) => {
                    print!("{} ", console::style(format!("{}", alph)).bold().yellow());
                }
                // Some(LetterState::Wrong) => {
                //     print!("{} ", console::style(format!("{}", alph)).bold().red());
                // }
                _ => {
                    print!("{} ", console::style(format!("{}", alph)).bold().black());
                }
            }
        }
    }

    pub fn print_wordle(&self) {
        // for guessed_word in &self.guessed_words {
        //     print!("{}\n", guessed_word);
        // }
        for guessed_word in &self.guessed_words {
            for alph in &mut guessed_word.chars() {
                // This is a mess, but it works, maybe compare with answer can be more elegible
                match self.used_letters.iter().find(|&letter| letter.alph == alph) {
                    Some(letter) => {
                        match letter.state {
                            LetterState::Correct => {
                                print!("{} ", console::style(format!("{}", alph)).bold().green());
                            }
                            LetterState::WrongSpot => {
                                print!("{} ", console::style(format!("{}", alph)).bold().yellow());
                            }
                            // LetterState::Wrong => {
                            //     print!("{} ", console::style(format!("{}", alph)).bold().red());
                            // }
                            _ => {
                                print!("{} ", console::style(format!("{}", alph)).bold().black());
                            }
                        }
                    }
                    _ => {
                        print!("{} ", console::style(format!("{}", alph)).bold().black());
                    }
                }
            }
            print!("\n");
        }
        self.print_alphabet();
    }
}