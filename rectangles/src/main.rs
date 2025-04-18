#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //The Self keyword in ther return type and in the body of the function are aliases for the type
    //that appears after the impl keyword.
    //In this case, Self is Rectangle
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle{
        width: 60,
        height: 45,
    };
    let sq1 = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width(){
        println!(
        "The rectangle has a nonzero width; it is {}", rect1.width
        );
    }
        
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("sq1 : {sq1:?}");


}
