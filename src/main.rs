use std::io;
use rand::Rng;
use std::cmp::Ordering;

// lottery parameters 
const MAX_NUMBER: u32 = 100; 
const MIN_NUMBER: u32 = 1;
const MAX_NUMBER_OF_GUESS: u32 = 3;


fn main() {
    // generate a random number (integer) in range [1-100] 
    let mut rng = rand::thread_rng();
    let winning_number: u32 = rng.gen_range(MIN_NUMBER..=MAX_NUMBER);
    println!("Winning number = {}", winning_number);
    
    // prompt user for lottery guess (3 tries max)  
    let mut number_of_tries = 0;
    let mut is_win: bool = false;
    
    while number_of_tries < MAX_NUMBER_OF_GUESS {
        // prompt user 
        println!("\nGuess a number between 1-100. {} out of 3 tries remaining.", 
            MAX_NUMBER_OF_GUESS - number_of_tries);

        let mut user_input: String = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
    
        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => {
                if in_valid_guess_range(num) {
                    num
                } else {
                    println!("Provided guess '{}' is out of valid range", user_input);
                    continue
                }
            }, 
            Err(_) => continue
        };

        // check against winning number 
        println!("You guessed: {user_input}");

        match user_input.cmp(&winning_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                is_win = true; 
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
        
        number_of_tries = number_of_tries + 1;
        
    }

    // Inform user of win/loss 
    match is_win {
        true => {
            println!("Winning number = {}", winning_number);
            println!("You win!");
        }
        false => println!("You get nothing! You lose! Good day sir!")
    }
}


fn in_valid_guess_range(x: u32) -> bool {
    if x >= MIN_NUMBER && x <= MAX_NUMBER {
        return true
    } else {
        return false
    }
}