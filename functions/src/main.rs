use std::io;

fn main() {
    println!("Enter Number:");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().parse().expect("Please type a number!");
    let result = odd_or_even(input);

    println!("Is the number even? {}", result);
}

fn odd_or_even(num: i32) -> bool{
    if num % 2 == 0 {
        true
    } else {
        false
    }
}