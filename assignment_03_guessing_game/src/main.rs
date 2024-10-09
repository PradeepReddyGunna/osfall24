fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0 
    } else if guess > secret {
        1 
    } else {
        -1 
    }
}

fn main() {
    let secret_number = 7;
    let mut attempts = 0;

    loop {
        let guess = attempts + 1; 
        attempts += 1;

        let result = check_guess(guess, secret_number);
        if result == 0 {
            println!("You guessed it! The number was: {}", secret_number);
            break; 
        } else if result == 1 {
            println!("Your guess of {} is too high.", guess);
        } else {
            println!("Your guess of {} is too low.", guess);
        }
    }
    println!("It took you {} attempts to guess the number.", attempts);
}
