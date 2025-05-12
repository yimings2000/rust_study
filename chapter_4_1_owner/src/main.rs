fn main() {
    // 4.1.2作用域
    // s不可用
    let mut _s = "hello"; //s 可用
    _s = "world";
    // 可以对s进行相关操作

    // 4.1.2 String类型定义,可变
    let mut s = String::from("Hello");
    s.push_str(", World");
    println!("{}", s);

    // 4.1.2 String - MOVE
    let s1 = String::from("hello");
    let _s2 = s1; // Move
    // println!("{}", s1); // s1已经无效,因为发生了move,不能再被borrow

    // lone, s3,s4都有效
    let s3 = String::from("Hello");
    let s4 = s3.clone();
    println!("{}, {}", s3, s4);

    // 4.1.3 所有权与函数
    let s5 = String::from("Hello World!");
    take_ownership(s5); // s5 把所有权传到函数中, s5失效
    // println!("{}", s5); // 所有权转移, 失效

    let x = 5;
    make_copy(x); // 拷贝了一个副本传到函数中
    println!("{}", x); // 依然有效

    // 返回值与作用域
    let _s6 = gives_ownership();
    let s7 = String::from("hello");
    let _s8 = takes_and_gives_back(s7); // s7把所有权传入函数,之后失效
} // s作用域结束, s不再可用

fn take_ownership(some_string: String) {
    println!("{}", some_string);
} // drop, some_string离开作用域

fn make_copy(some_number: i32) {
    println!("{}", some_number);
} // 什么都不会发生

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
} // 所有权传出

fn takes_and_gives_back(a_string: String) -> String {
    a_string
} // 函数的作用是取得传入参数的所有权, 再返回出去

