#[derive(Debug)]
pub struct Rectangle {
    lenght: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.lenght > other.lenght && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 3
}

pub fn greeting(name: &str) -> String {
    format!("Hello!")
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value)
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value)
        }

        Guess { value: value }
    }
}

// 私有函数
pub fn add_twos(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;  // 把上层的结构体和实现引用进来

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail")
    }

    // assert!测试
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            lenght: 8,
            width: 7,
        };
        let smaller = Rectangle {
            lenght: 5,
            width: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    // assert_wq!
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    // ---- tests::it_adds_two stdout ----
    // thread 'tests::it_adds_two' panicked at 'assertion failed: `(left == right)`
    // left: `4`,
    // right: `5`', src\lib.rs:62:9

    // 自定义错误信息assert
    #[test]
    fn greetings_contain_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), 
        "Greeting didn't contain name, value was '{}'",
        result);
    }
    // ---- tests::greetings_contain_name stdout ----
    // thread 'tests::greetings_contain_name' panicked at 'Greeting didn't contain name, value was 'Hello!'', src\lib.rs:77:9

    // shold_panic 测试
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    #[test]
    #[should_panic(expected="Guess value must be less than or equal to 100")]  // 期待发生恐慌包含的字符串
    fn greater_than_200() {
        Guess::new(200);
    }

    // 使用Result<T, E>测试
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("twi plus two does not equal four"))
        }
    }

    // ignore属性, 忽略测试     test tests::it_adds_three ... ignored
    // cargo test -- --ignored
    #[test]
    #[ignore]
    fn it_adds_three() {
        assert_eq!(4, add_two(2));
    }

    // 测试私有函数
    #[test]
    fn it_private_works() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
