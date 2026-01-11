fn main() {
    let r = Rectangle {
        height: 30,
        width: 50,
    };
    let a = area(&r);
    println!("{:#?}", r);
    println!("a = {}", a);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
