use std::net::IpAddr;
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
fn main() {
    // 使用unwrap, 不适用panic
    let home: IpAddr = "127.0.0.1".parse().unwrap(); // "127.0.0.1".parse()肯定不会出现panic
    loop {
        // ...
        let guess = "32";
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => {
                println!("{}", num);
                num
            },
            Err(_) => continue,
        };

        let guess = Guess::new(guess);
    }
}
