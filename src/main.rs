// import commonly used items from the prelude:
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    
    // println!("The secret number i6s: {}", secret_number);
    
    println!("Please input your guess.");

    let mut counter: i32 = 0;

    loop {
        if counter != 0 {
            println!("Try again!");
        }
        let mut guess = String::new();
       
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        if counter == 10 {
            println!("You failled!");
            break;
        }
        counter += 1;

    }
}