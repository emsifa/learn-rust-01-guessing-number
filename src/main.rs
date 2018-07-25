extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("---- Guess the number ----");

    let secret_number = rand::thread_rng().gen_range(1, 100);
    let mut trial = 0;
    let max_trial = 5;

    loop {
        if trial == max_trial {
            println!("You loose!");
            break;
        }

        let mut guess = String::new();
        let life = max_trial - trial;
        let fails = trial;

        println!("[{}{}] Guess the number: ", "-".repeat(fails), "o".repeat(life));
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        let guess: u32 = guess.trim().parse().expect("Please type a number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        trial += 1;
    }
}
