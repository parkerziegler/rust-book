// Import standard lib io to obtain user input.
use std::io;
// Import standard lib cmp to compare values.
use std::cmp::Ordering;
// Import rand for random number generation.
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Get the random number generator for the thread instance.
    // Then, generate a range b/t 1 and 100; the upper bound is exclusive.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please enter your guess.");

        // Define a mutable variable for storing the user's guess.
        // new is an _associated function_ of the String type.
        let mut guess = String::new();

        io::stdin()
            // Capture user input and assign it to guess.
            // &mut signals the assignment is happening on a mutable _reference_ to guess.
            .read_line(&mut guess)
            // Handle the io::Result variant with expect.
            // This will return the enclosed String if the io::Result is an Err.
            .expect("Failed to readline");

        // Shadow the value of guess with the u32 representation.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Print the string using a placeholder.
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
