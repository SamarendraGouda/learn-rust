use std::io;
fn main() {
    println!("Guess a number.");
    println!("Enter Your Guess:");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read a line");

    println!("{guess}");
}
