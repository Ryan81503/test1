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
    let secret_number = 42;
    let mut guess_count = 0;

    loop {
        let mut guess = String::new();
        println!("Enter your guess: ");
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: i32 = guess.trim().parse().expect("Please type a number!");
        guess_count += 1;

        match check_guess(guess, secret_number) {
            0 => {
                println!("Correct! It took {} guesses.", guess_count);
                break;
            }
            1 => println!("Too high!"),
            -1 => println!("Too low!"),
            _ => (),
        }
    }
}