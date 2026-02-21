use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    println!("some changes to se wath is uploaded and wath not.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}