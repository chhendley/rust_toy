// src/main.rs
use std::io;
use rand::Rng;

fn generate_random_number() -> u32 {
    rand::rng().random_range(1..=100)
}

fn read_user_input() -> u32 {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    user_input.trim().parse().expect("Please enter a valid number")
}

fn compare_numbers(user_input: u32, random_number: u32) {
    if user_input < random_number {
        println!("Your guess is too low!");
    } else if user_input > random_number {
        println!("Your guess is too high!");
    } else {
        println!("Congratulations! You've guessed the number!");
    }
}

fn main() {
    println!("Welcome to the number guessing game!");
    println!("I have selected a random number between 1 and 100.");
    println!("Can you guess what it is? You have 10 attempts.");

    let random_number = generate_random_number();
    let mut attempts = 0;

    loop {
        let user_input = read_user_input();
        if user_input < 1 || user_input > 100 {
            println!("Please enter a number between 1 and 100.");
            continue;
        }
        attempts += 1;
        if attempts == 1 {
            println!("You have 10 attempts to guess the number.");
        }

        if attempts > 10 {
            println!("Sorry, you've used all your attempts! The number was: {}", random_number);
            break;
        }
        println!("You guessed: {}", user_input);
        compare_numbers(user_input, random_number);
        
        if user_input == random_number {
            println!("You guessed the number in {} attempts!", attempts);
            break;
        }
    }
}
// This is a simple number guessing game where the user has to guess a random number between
// 1 and 100. The user has 10 attempts to guess the number, and the program provides feedback on whether the guess is too high or too low. 
// If the user guesses the number correctly, the game congratulates them and displays the number of attempts taken. If the
