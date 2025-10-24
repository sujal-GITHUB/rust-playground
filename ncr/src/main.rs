use std::io;

fn ncr(n:u32, r:u32) -> u32{
    let mut res = 1;

    (0..r).for_each(|i| {
        res *= n-i;
        res /= i+1;
    });

    res
}

fn main() {
    println!("Enter n: ");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = n.trim().parse().expect("Please type a number!");

    println!("Enter r: ");
    let mut r = String::new();
    io::stdin().read_line(&mut r).expect("Failed to read line");
    let r: u32 = r.trim().parse().expect("Please type a number!");

    let ans = ncr(n, r);

    println!("The value of {}C{} is {}", n, r, ans);
}
