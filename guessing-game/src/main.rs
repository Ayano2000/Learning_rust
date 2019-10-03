use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    // setting the number user is trying to get.
    let secret_number = rand::thread_rng().gen_range(1, 100);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        // declaring a guess variable.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        // setting the users input to be the guess variable.
        // exception if the line read fails.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        // creates a "shadow of the guess variable, 
        // u32 means only numeric, trim removes and whitespace
        // parse converts to a numeric vablue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            // takes the input and checks it agianst the random number
            // generated in line 8, tells you too big or small and exits
            // when the correct number is input.
            }
        }
    }
}