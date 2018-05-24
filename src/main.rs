extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
// 

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("guess the number!");
    println!("The secret number is {}!", secret_number);
    println!("Please input your guess.");

    loop {
        if guess_no(secret_number) {
            break
        }
    }
}

fn guess_no(secret_number: u32) -> bool {
    let mut guess = String::new();
    // mut ~ mutable
    // String UTF-8
    // ::new() `::`一个特类型的关联函数，类方法？

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_)  => 0, // 不管任何错误都...
    };

    println!("You guess: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("You win");
            return true
        },
    }
    false
}