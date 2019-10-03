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

// Keeps track of the discovered letters the word the lives remaining
// games status
struct GameData {
    discovered_letters      : String,
    secret_line             : String,
    lives                   : i32,
    status                  : String,
}

// Used to keep track of the users input letters.
enum UserInputStatus {
    AlreadyDiscovered,
    LetterGuessed,
    LetterMissed,
}

fn main()
{
    let random_line = get_line().expect("Failed to read input data!");

    let mut gd : GameData = GameData {
                        secret_line        : random_line,
                        discovered_letters : String::new(),
                        lives              : 5,
                        status             : String::new()
                        };

    let mut secret_line_masked = format_hidden_string(&gd.secret_line, &gd.discovered_letters);
    // we use this string to check the guesses against the letters present in the string.
    loop
    {
        refresh_screen(&gd, &secret_line_masked);
        println!("Type your guess:");
        let user_guess = read_guess();
        // user guess will be re-assigned to be what the latest one is.
        if validate_user_guess(user_guess)
        {
            let guess_lower = user_guess.unwrap().to_lowercase().next().unwrap();
            // guess lower is the users guess checked and converted to lowercase.
            match check_user_guess(&gd, guess_lower)
            {
                //this checks the users input and then executes based on whether
                // the letter is in the string, not in the sting or previousely guessed.
                UserInputStatus::LetterGuessed =>
                {
                    gd.discovered_letters.push(guess_lower);
                    let status = format!("You discovered {}", guess_lower);
                    gd.status = Green.paint(status).to_string();
                    secret_line_masked = format_hidden_string(&gd.secret_line, &gd.discovered_letters);

                    if !secret_line_masked.contains('_')
                    {
                        gd.status = Green.bold().paint("You won!").to_string();
                        refresh_screen(&gd, &secret_line_masked);
                        break;
                    }
                }

                UserInputStatus::LetterMissed =>
                {
                    gd.discovered_letters.push(guess_lower);
                    gd.lives = gd.lives - 1;

                    if gd.lives == 0
                    {
                        gd.status = Red.bold().paint("You lost!").to_string();
                        secret_line_masked = format_hidden_string(&gd.secret_line, &gd.secret_line);
                        refresh_screen(&gd, &secret_line_masked);
                        break;
                    }
                    else
                    {
                        let status = format!("Unfortunately, no {}", guess_lower);
                        gd.status = Red.paint(status).to_string();
                    }
                }

                UserInputStatus::AlreadyDiscovered =>
                {
                    let status = format!("{} is already discovered!", guess_lower);
                    gd.status = Yellow.paint(status).to_string();
                }
            }
        }
        else
        {
            let status = format!("It is not a letter!");
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
    print_hangman(&gd);
    // might not print a hangman, just want to digest the code.
    println!("{}", secret_line);
    println!("{}", gd.status);
}

fn read_guess() -> Option<char>
{
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    guess.trim().chars().nth(0)
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

fn print_hangman(gd: &GameData)
{
    match gd.lives
    {
        0 =>
        {
            println!(" _________   ");
            println!("|         |  ");
            println!("|         XO ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|            ");
            println!("|            ");
        }

        1 =>
        {
            println!(" _________   ");
            println!("|         |  ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|        ||| ");
            println!("|        ||| ");
        }

        2 =>
        {
            println!(" _________   ");
            println!("|            ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|        ||| ");
            println!("|        ||| ");
        }

        3 =>
        {
            println!(" _________   ");
            println!("|            ");
            println!("|            ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|        ||| ");

        }

        4 =>
        {
            println!(" _________   ");
            println!("|            ");
            println!("|            ");
            println!("|            ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
        }

        _ =>
        {
            println!("             ");
            println!("             ");
            println!("             ");
            println!("             ");
            println!("          O  ");
            println!("         /|\\ ");
            println!("         / \\ ");
        }
    }
}