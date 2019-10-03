use std::io;

fn  main() {
    println!("Give the plebs a word to guess!");
    let mut word = String::new();

    io::stdin().read_line(&mut word)
        .expect("Failed to find word");
    println!("Guess a letter!");
    let mut x: u8 = 10;
    
    println!("{} tries remaining!", x);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to find letter");
    let len: usize = word.chars().count();
    println!("SH {}", len);
    let user_word_guesses = vec![0, len];
    println!("{}", user_word_guesses[1]);
}