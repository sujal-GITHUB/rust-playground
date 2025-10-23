struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.height * self.width
    }
}

fn main(){
    let rect = Rectangle{
        height: 30,
        width: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rect.area());
}