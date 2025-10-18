// Without slices, we would have to return the index of the first space

fn main() {
    let input = "Helloworld!";

    let idx = return_space(input);
    println!("Index of first space: {}", idx);
}

fn return_space(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Slices allow us to reference a contiguous sequence of elements in a collection

fn return_space_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
