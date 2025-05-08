const MAX_POINTS: u32 = 10_000;
fn main() {
    //let and mut
    let mut x = 5;
    println!("the value of x is {}", x);
    x = 6;
    println!("the value of x is {}", x);

    // constant
    // const MAX_POINTS: u32 = 10_000;
    println!("max points is {}", MAX_POINTS);

    // shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("the value of y is {}", y);

    let spaces = "    "; //&str
    let spaces = spaces.len(); // usize
    println!("{}", spaces);

    // 由于42.parse() 可以解析出来多种类型，所以必须指定一种(u32 或者 i32)
    let guess: u32 = "42".parse().expect("not a number");
    println!("{}", guess);

    // 浮点类型
    // help: if this is intentional, prefix it with an underscore: `_f1`
    let _f1 = 2.0;  // 不标注类型 ，默认f64
    let _f2: f32 = 3.0;  // 标注类型，f32

    // 加减乘除取余
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _reminder = 54 % 5;
}
