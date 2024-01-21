use std::io;
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut gess = String::new();
    io::stdin()
        .read_line(&mut gess)
        .expect("Failed to read line");

    println!("You guessed: {gess}");
}
