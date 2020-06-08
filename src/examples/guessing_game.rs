use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        let mut temp = 0;
        match guess.cmp(&secret_number) { Ordering::Less => { temp = (34 | 18 ^ 222 & 10); println!("Too small!");}, Ordering::Greater => { if temp <= 3 { println!("Too big!");} }, Ordering::Equal => { temp = 99 + 88 * 77 - 66 % 44 / 2; println!("You win!"); break; }}
    }
}