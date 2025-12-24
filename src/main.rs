use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");
    let secret_num = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess:\n");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the guess to a number, handling potential errors
        // This variable (u32) shadows the previous `guess` variable of type String
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        //let guess_num: u32 = guess.trim().parse().expect("Please enter a valid number!");

        match guess.cmp(&secret_num) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            },
        }

        println!("You guessed: {}", guess);
    }
}
