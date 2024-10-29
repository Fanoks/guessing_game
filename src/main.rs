use std::io;
use rand::Rng;


fn main() -> () {
    let stdin: io::Stdin = io::stdin();
    let mut input: String = String::new();
    
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Give your guess: ");
        input.clear();
        stdin.read_line(&mut input).unwrap();

        match input.trim().parse::<u8>() {
            Ok(guess) => {
                if guess < secret_number {
                    println!("Too small!");
                } else if guess > secret_number {
                    println!("Too big!");
                } else {
                    println!("You won!");
                    break;
                }
            }
            Err(_) => {
                println!("Please enter a valid number!");
            }
        }
    }
}
