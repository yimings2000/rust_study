fn main() {
    let some_number = Some(5);
    let some_string = Some("A String");

    let absent_number: Option<i32> = None; // 编译器无法自动推断,需要显示声明

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // Option<i8>和i8无法直接相加
}
