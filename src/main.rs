use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess an integer between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("šš¼ā Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess == "quit" {
            println!("Goodbye... šš»");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\n===========================================\nš§ Input must be a integer below 1 and 100!\n===========================================\n\n");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\nš¤­ Too low!\n-----------\n"),
            Ordering::Greater => println!("\nš² Too high!\n-----------\n"),
            Ordering::Equal => {
                println!("\n======================\n\nš Perfect! You win š\n\n\n");
                break;
            }
        }
    }
}
