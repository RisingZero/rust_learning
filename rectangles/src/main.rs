#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    // First, two variables
    let width1 = 10;
    let height1 = 20;
    println!("Width 2 variables");
    println!("The area of the rectangle is {} square pixels", area1(width1,height1));

    // With tuples
    let rect2 = (10,20);
    println!("Width tuples");
    println!("The area of the rectangle is {} square pixels", area2(rect2));

    // With structs
    let rect3 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("With structs");
    println!("The area of the rectangle is {} square pixels", area3(&rect3));
    println!("Rectangle is {:#?}", rect3); // rect3 is still valid because we passed reference, print debug information implemented with the derive Debug
    let rect3 = dbg!(rect3);
    // or
    dbg!(&rect3); // dbg! doesn't take ownership

    // With method
    let rect4 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("The area of the rectangle is: {}", rect4.area()); // Automatic referencing (and dereferencing) donoe by Rust

    
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}