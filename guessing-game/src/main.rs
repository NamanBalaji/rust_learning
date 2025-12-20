use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let number = input.trim_end().parse::<i32>().unwrap();

        match number.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("You guessed too low"),
            std::cmp::Ordering::Equal => {
                println!("You got it !!!!");
                return;
            }
            std::cmp::Ordering::Greater => println!("You guessed too high"),
        }
    }
}
