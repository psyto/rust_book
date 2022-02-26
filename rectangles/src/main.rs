fn main() {
    let height1 = 5;
    let width1 = 8;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(height1, width1)
    );
}

fn area(height: u32, width: u32) -> u32 {
    height * width
}
