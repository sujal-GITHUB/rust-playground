fn main() {
    s = String::from("Sujal");

    let len = calculate_length(s);
    
    // s is no longer valid here because its ownership has been moved to the function
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: String) -> usize {
    s.len()
}

// References and Borrowing

fn main() {
    s = String::from("Sujal");

    let len = calculate_length(&s);
    
    // s is still valid here because we passed a reference to the function
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}