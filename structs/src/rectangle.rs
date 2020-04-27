
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

pub fn demo_rectangle() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // NB this doesn't work if we move rather than borrow when passing struct to the area function
    println!("The original rectangle is -  {:?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
