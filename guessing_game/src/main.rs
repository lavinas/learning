use rand::Rng;
use std::cmp::Ordering;
use std::io;

// main function
fn main() {
    let secret_number = random();
    let mut win = false;
    while !win {
        let guess = get();
        if guess == -1 {
            continue;
        }
        win = check(guess, secret_number);
    }
}

// generate a random number
fn random() -> i32 {
    rand::thread_rng().gen_range(1..=100)
}

// get a number from the user
fn get() -> i32 {
    println!("Guess the number!");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number!");
            -1
        }
    };
    guess
}

// check if the guess is correct
fn check(guess: i32, secret_number: i32) -> bool {
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!");
            false
        }
        Ordering::Greater => {
            println!("Too big!");
            false
        }
        Ordering::Equal => {
            println!("You win!");
            true
        }
    }
}
