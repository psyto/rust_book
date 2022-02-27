#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 40,
    };

    let rect2 = Rectangle {
        height: 20,
        width: 30,
    };

    let rect3 = Rectangle {
        height: 40,
        width: 30,
    };

    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
