fn main() {
    let s = String::from("नमस्ते");

    // Bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    for b in s.bytes() {
        println!("{}", b);
    }

    // Chars
    // ['न', 'म', 'स', '्', 'त', 'े']\

    for c in s.chars() {
        println!("{}", c);
    }

    // Grapheme Clusters
    // ["न", "म", "स्", "ते"]

}
