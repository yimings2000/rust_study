
fn plus_five(x: i32) -> i32 {
    // 注意：加了分号就变成语句，就不是表达式，返回就有错误
    x + 5
}

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 解构
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);
    // 点标记法
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    // 数组
    let _a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "Augest",
        "Septmber",
        "October",
        "November",
        "December",
    ];

    // 访问数组中的元素
    let _first = months[0];
    let _second = months[1];

    // 越界，编译不会报错，运行报错
    let _index = [12, 13, 14, 15];
    // let _month = months[_index[1]];

    let x = plus_five(6);
    println!("the value of x is {}", x);
}
