fn main() {
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    let mut v2 = vec![4, 5, 6];

    let does_not_exist = v.get(2);

    for i in &mut v2 {
        *i += 1;
    }

    println!("Vector: {:?}", v2);
    println!("Does not exist: {:?}", does_not_exist);
}
