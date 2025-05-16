#[derive(Debug)] // 为结构体显示的选择debug功能
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    // struct 的方法
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, length: size}
    }
}
fn main() {
    let _s = Rectangle::square(20); // 调用关联函数用::
    let _w = 30;
    let _l = 50;

    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        length: 40,
    };
    let rect3 = Rectangle {
        width: 35,
        length: 55,
    };

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));
}
