#[derive(Debug)]

enum IPAddressKind{
    V4,
    V6,
}

struct IPAddress{
    kind: IPAddressKind,
    address: String,
}

fn main() {
    let google = IPAddress{
        kind: IPAddressKind::V4,
        address: String::from("1.2.3.4"),
    };

    println!("Google IP Address: {:?} {}", google.kind, google.address);
}
