struct Dimensions{
    width: u32,
    height: u32,
}

fn main() {
    let rec1 = Dimensions {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} sq. px.", calculate_area(&rec1));
}

fn calculate_area(rec1: &Dimensions) ->u32{
    rec1.width * rec1.height
}
