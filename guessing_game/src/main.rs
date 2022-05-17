use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Guessing Game");
    
    println!("");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please guess a number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = guess.trim().parse().expect("Invalid number");

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
            Ordering::Greater => println!("Too big"),
        }
    }
    
}
