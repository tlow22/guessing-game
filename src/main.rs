use std::io;
use rand::Rng;
fn main() {
    // generate a random number (integer) in range [1-100] 
    let mut rng = rand::thread_rng();
    let winning_number: u32 = rng.gen_range(1..=100);
    println!("Winning number = {}", winning_number);

    // prompt user for lottery guess   
    println!("Guess a number between 1-100");
    let mut user_input: String = String::new();
    
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    
    println!("You guessed: {user_input}")

    
}
