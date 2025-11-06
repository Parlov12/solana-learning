use std::io; // importing io(input/output) library which is part of standard library std
// prelude - name for items that come by default with std

use rand::Rng; // importing trait Rng which defines methods that random number generator implements
// trait Rng must be in scope for us in order to use those methods

use std::cmp::Ordering; // to compare elements


fn main() {

    println!("Guess the number!");

    let secret_number  = rand::thread_rng().gen_range(1..=100);
    // rand::thread:_rng - function that gives us the particular random number generator
    // we are using one that is local to the currebt thread of execution and is seeded by the operating system
    // gen_range method takes a range expression as an argument and generates a random number in the rang

    println!("The secret number is: {secret_number}");

    println!("Please input your guess:");

    let mut guess = String::new();

    // mut - means variable is mutable
    // variable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!"); // handle error


    println!("You guessed : {}", guess);
    // &mut guess - & means it is reference

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"), // each single one of those is called arm
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win"),
    }


}
