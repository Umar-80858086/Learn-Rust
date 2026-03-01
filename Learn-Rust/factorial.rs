use std::io;

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: u64 = input.trim().parse().expect("Please enter a valid number");

    let mut factorial: u64 = 1;

    for i in 1..=number {
        factorial *= i;
    }

    println!("Factorial of {} is {}", number, factorial);
}