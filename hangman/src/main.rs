extern crate rand;
extern crate ansi_term;

use std::process::Command;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use rand::{thread_rng, sample};

use ansi_term::Colour::Red;
use ansi_term::Colour::Green;
use ansi_term::Colour::Yellow;

struct GameData {
    discovered_letters      : String,
    secret_line             : String,
    lives                   : i32,
    status                  : String,
}

fn  main() {
    let     to_guess = get_line().expect("Failed to find line!");

    let mut gd : GameData = GameData {
        secret_line         : to_guess,
        discovered_letters  : String::new(),
        lives               : 5,
        status              : String::new(),
    };

    let mut secret_line_masked = format_hidden_string(&gd.secret_line, &gd.discovered_letters);
    // will have no discovered letters in this call.
    // will create the secret_line_masked string.
    loop {
        // refresh_screen(&gd, &secret_line_masked);
        println!("Well give it a go");
        let user_guess = read_guess();

        if validate_user_guess(user_guess) {
            let guess_lower = user_guess.unwrap().to_lowercase().next().unwrap();
            match check_user_guess(&gd, guess_lower) {
                UserInputStatus::LetterGuessed => {
                    gd.discovered_letters.push(guess_lower);
                    let status = format!("You discovered: {}", guess_lower);
                    gd.status = Green.paint(status.to_string());
                    secret_line_masked = format_hidden_string(&gd.secret_line, &gd.discovered_letters);
                    if !secret_line_masked.contains('_') {
                        gd.status = Green.bold().paint("You won!").to_string();
                        refresh_screen(&gd, &secret_line_masked);
                        break ;
                    }
                    else {
                        let status = format!("Unlucky mate {}", guess_lower);
                        gd.status = Red.paint(status).to_string();
                    }
                }
                UserInputStatus::AlreadyDiscovered => {
                    let status = format!("{} has already been discovered!", guess_lower);
                    gd.status = Yellow.paint(status).to_string();
                }
            }
        }
        else {
            let status = format!("It's not a letter!");
            gd.status = Yellow.paint(status).to_string();
        }
    }
}

fn  get_line() -> Result<String, io::Error> {
    let f = r#try!(File::open("words.txt"));
    // #r to workaround the try keyword being reserved.
    let file = BufReader::new(&f);
    let mut rng = thread_rng();
    let sample = sample(&mut rng, file.lines(), 1).pop().unwrap();
    let secret_line = sample.unwrap().to_lowercase();
    // sets secret_line to be what ever line the rng gives us
    // and converts it to lowercase.
    Ok(secret_line)
}

fn  format_hidden_string(input: &String, mask: &String) -> String {
    let mut result : String = String::new();
    
    for (i, c) in input.chars().enumerate() {
        result.push(if c == ' ' {c}
            else if mask.contains(c) {c}
            else {'_'});
        result.push(' ');
    }
    // checks if the char c is present in the mask string.
    // if it is it sets it in the correct position in the result string.
    result
}

fn  refresh_screen(gd: &GameData, secret_line: &String) {
    println!("{}[2J", 27 as char);
    println!("Can you guess the word?");
    println!("Lives Remaining: {}. Letters Discovered: {}", gd.lives, gd.discovered_letters);
    // print_hangman(&gd);
    // might not print a hangman, just want to digest the code.
    println!("{}", secret_line);
    println!("{}", gd.status);
}

fn  read_guess() -> Option<char> {
    let mut guess = String::new();
    io::stdin.read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().chars().nth(0)
    // reads the guess from the stdin.
    // removes any bad chars before returning the users input.
}

fn  validate_user_guess(user_guess: Option<char>) -> bool {
    // takes guess and makes sure it an alphabetic char.
    // if its not it returns nothing.
    match user_guess {
        Some(guess) => {
            if !guess.is_alphabetic() { false }
            else { true }
        }
        none => { return false; }
    }
}

fn  check_user_guess(gd: &GameData, user_guess: char) -> UserInputStatus {
    if gd.discovered_letters.contains(user_guess) {
        return UserInputStatus::AlreadyDiscovered;
    }
    if !gd.secret_line.contains(user_guess) {
        return UserInputStatus::LetterMissed;
    }
    UserInputStatus::LetterGuessed
}