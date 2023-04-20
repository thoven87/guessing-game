use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    // Plan for the game
    // 01 - Generate a secret number
    let secret = rand::thread_rng().gen_range(1..101);
    // let secret = 22;
    // 02 - Ask user number
    println!("Guess the number (hint {})", secret);

    loop {
        // 03 - Read user number as String
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        // 04 - Verify user number and display to user
        let guess: i32 = guess
            .trim()
            .parse()
            .expect("failed to read the number, please type a number.");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too high"),
            Ordering::Equal => {
                println!("you won!!!");
                break;
            }
        }
    } // 05 - loop on 3 and 4

    println!("\nGoodye! It was fun playing guess the number.\n") // 06 - Goobye message
}
