use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_number() {
    println!("Guess the number:");
    let mut guess: String = String::new(); // :: calls a static method for type (class) String.
    io::stdin()
        .read_line(&mut guess)// & indicates the argument is a reference
        .expect("failed to read line");

    let secret_number: u32 = rand::thread_rng().gen_range(1..100);
    // convert the string guess to integer
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        _ => {}
    }

    println!("your guess is {}", guess);
    println!("the secret number is {}", secret_number);
}

// allow this piece of code be unused, silent warnings from the compiler
#[allow(dead_code)]
// this is a private method by default
fn test_mut() {
    let mut x = 5;
    println!("value is {}", x);
    x = 6;
    println!("value is {}", x);
}
