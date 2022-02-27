#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rec1 = Rectangle {
        height: 30,
        width: 40,
    };

    println!("rect1 is {:?}", rec1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rec1.area()
    );
}
