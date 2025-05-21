enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // 创建vector 1
    // let v = Vec::new(); // new是创建一个空的vector, 所以需要指定vector的类型<i32>
    // vec!宏初始值创建vector，编译器可以推断出类型
    let v = vec![1, 2, 3, 4, 5];
    // 没添加元素前无法推断类型会报错
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    // 读取Vector里的值
    // 1.
    let third = &v1[2]; // &v[100] 越界非法访问,会报错
    println!("The third element is {}", third);
    // 2.
    match v.get(2) {  // v.get(100) 越界，走None分支
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // 所有权
    let mut v2 = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // 不可变借用
    // v.push(6);  // 可变引用, 与不可变借用不可同时存在
    println!("The first element is {}", first);

    // 遍历Vector里的值
    // 1. 不可变引用
    let v3 = vec![100, 50, 33];
    for i in &v3 { // i:&i32
        println!("{}", i);
    }
    // 2. 可变引用
    let mut v4 = vec![10, 5, 3];
    for i in &mut v4 {
        *i += 50; // 解引用,再使用
    }
    for i in v4 { // i:i32
        println!("{}", i);
    }

    // 通过enum在Vector存储多种类型
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

} // 出作用域,所有vector都被清理掉
