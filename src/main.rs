
mod util;
mod comple_type;
mod array;
mod controlle_flow;
mod r#match;
mod r#trait;
mod collections;

// use std::io;
// use rand::Rng;
// use crate::util::add_three;

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
    // let a = 12;
    // let b = a;
    //
    // let s1= String::from("hello world!");
    // let s2 = s1.clone();
    //
    //
    // println!("{s1},{s2}");
    //
    //
    // let b = add_three(3);
    // println!("add three is {b}");

    let x = 5;
    let y = &x;

    assert_eq!(5,x);
    assert_eq!(5,*y);


}


