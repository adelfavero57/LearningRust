use std::io;
use rand::Rng;
use std:: cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    
    loop {

        println!("Please input your guess.");
        

        //"let" creates the variable
        //"mut" makes the variable mutable (variables are immutable by default)
        //"::" types the function
        let mut guess = String::new();
        
        //Receives user input
        //".read_line" reads the user's input and stores it in the "guess" variable
        //since we are writing to a reference, we need to include "mut", since the reference is
        //immutable by default
        //The output of .read_line is a Result enum. If this enum is "err", the .expect function will
        //catch it and output the string passed.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //Reusing the variable name is called "Shadowing", often used when converting variable types.
        //We handle errors with the match expression, which returns result enum. "continue" will
        //allow the guesser to re-attempt
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error, not a number");
                continue;
            }
        };
     
        println!("You guessed: {}", guess);
        
        //cmp method called on 'guess'. Takes argument for comparrison. We dereference the pointer to
        //secret number.
        // The 'match' keyword allows us to decide what to do based on all possible outcomes of the
        // comparrison method
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }

        }
    }
            
} 
