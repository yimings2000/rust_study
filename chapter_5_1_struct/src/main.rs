#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };

    println!("area is {}", area(&rect1));

    println!("Rectanle is {:?}", rect1);
    println!("Rectanle is {:#?}", rect1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.length * rect.width
}
