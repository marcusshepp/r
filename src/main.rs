use std::io;
use rand::Rng;

fn main() {
    println!("Guess: ");
    let sec_num = rand::thread_rng().gen_range(1..=100);
    println!("sec: {sec_num}");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("failed to readline");

    println!("you guessed: {}", guess);
}
