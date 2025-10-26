fn main() {
    let a = 10.0;
    let b = 6.0;

    match divide(a, b) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("{}", e),
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Error: Division by zero"))
    } 
    else {
        Ok(a / b)
    }
}