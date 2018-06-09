extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!("Guess the number!");
  println!("Please Input your guess:");
  let secret_number = rand::thread_rng().gen_range(1, 101);
  loop {
    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read the line");
    println!("You've guesses: {}", guess);

    let guess: u32 = match guess.trim().parse(){
      Ok(num) => num,
      Err(_) => {
        println!("Please enter a number");
        continue;
        }
    };
   

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too Small"),
      Ordering::Greater => println!("Too Big"),
      Ordering::Equal => {
        println!("You Win");
        break;
        },
    }
  }
}
