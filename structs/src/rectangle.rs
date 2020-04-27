use std::fmt;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Rectangle - width: {} pixels, height: {} pixels, area: {} square pixels",
            self.width,
            self.height,
            self.area()
        )
    }
}

pub fn demo_rectangle() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The struct is: {}", rect1);
}

