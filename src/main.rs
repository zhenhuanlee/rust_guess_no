extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
// 

fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}!", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();
    // mut ~ mutable
    // String UTF-8
    // ::new() `::`一个特类型的关联函数，类方法？

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");

    let guess: u32 = guess.trim().parse().
        expect("please type a number");

    println!("You guess: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win"),
    }
}
