#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 绑定值
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main() {
    let c = Coin::Quarter(UsState::Alaska); // 绑定值得模式匹配
    println!("{}", value_in_cents(c));

    // 匹配Option<T>枚举
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // _通配符匹配
    let v = 0u8;
    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // 只匹配一种情况
    let v = Some(0u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }
    // if let 更简单
    if let Some(3) = v {
        println!("three");
    } else {
        println!("others");
    }
}
