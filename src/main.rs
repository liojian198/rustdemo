
mod util;

use std::io;
use rand::Rng;
use crate::util::add_three;

fn main() {
    // println!("guess the number!");
    // let secret_number = rand::thread_rng().gen_range(1..=100);
    //
    //
    // println!("Please input your guess.");
    // let mut  guess:String = String::new();
    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("failed to read line");
    // println!("your guessed: {guess}, the secret num is {secret_number}");
    let b = add_three(3);
    println!("add three is {b}");

}
