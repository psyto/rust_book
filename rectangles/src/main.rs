#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let rec1 = Rectangle {
        height: 30,
        width: 40,
    };

    println!("rect1 is {:?}", rec1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rec1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
