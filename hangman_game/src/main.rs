use std::io;
use std::fs;
use std::process;
use rand::seq::SliceRandom;

fn main() {
    loop {
        println!("Shall we play a round of hangman? [Y/N]");
        let mut user_start = String::new();
        io::stdin().read_line(&mut user_start).expect("Failed to read input.");
        let user_start = user_start.trim().to_uppercase();
        if user_start == "Y" {
            let word = select_word("short_dictionary.txt");
            game(&word);
        } else if user_start == "N" {
            process::exit(0);
        }
    }
}

fn select_word(file_name: &str) -> String {
    let path = format!("Data/{}", file_name);
    match fs::read_to_string(&path) {
        Ok(contents) => {
            let words: Vec<&str> = contents.split_whitespace().collect();
            let mut rng = rand::thread_rng();
            let word = words.choose(&mut rng).expect("Dictionary is empty.");
            String::from(*word)
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(1);
        }
    }
}

fn game(word: &str) {
    let mut guessed_letters = vec!['-'; word.len()];
    let mut remaining_guesses = 6;
    let mut guessed = false;

    println!("Welcome to Hangman!");

    while !guessed && remaining_guesses > 0 {
        println!("Word: {}", guessed_letters.iter().collect::<String>());
        println!("Guesses remaining: {}", remaining_guesses);
        println!("Enter a letter to guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input.");

        let guess: char = match guess.trim().chars().next() {
            Some(c) => c,
            None => continue,
        };

        if word.contains(guess) {
            println!("Correct!");
            for (i, letter) in word.chars().enumerate() {
                if letter == guess {
                    guessed_letters[i] = guess;
                }
            }
        } else {
            println!("Incorrect!");
            remaining_guesses -= 1;
        }

        if !guessed_letters.contains(&'-') {
            guessed = true;
        }
    }

    if guessed {
        println!("Congratulations! You guessed the word: {}", word);
    } else {
        println!("Sorry, you ran out of guesses. The word was: {}", word);
    }
}
