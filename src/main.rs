use std::io;

fn main() {
    println!("Guess a number between 1-100");
    let mut user_input: String = String::new();
    
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    
    println!("You guessed: {user_input}")
}
