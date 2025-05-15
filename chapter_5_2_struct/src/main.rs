#[derive(Debug)]  // 为结构体显示的选择debug功能
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    let w = 30;
    let l = 50;
    println!("{}", area1(w, l));

    let rect = (30, 50);
    println!("{}", area2(rect));

    let rect = Rectangle {
        width: 30,
        length: 50
    };
    println!("{}", area3(&rect)); // 借用

    println!("{:#?}", rect); // struct没有实现display，无法直接打印
    // {:?} {:#?}更易读
}

// 版本1, width与length没有联系
fn area1(width: u32, length: u32) -> u32 {
    width * length
}

// 版本2, 放在一个元组里, 可读性差
fn area2(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

// 版本3, 放在一个结构体里, 既有联系, 可读性又高
fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}