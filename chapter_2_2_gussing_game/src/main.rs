use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Gusse the game!");

    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("secret num is {}", secret_num);

    loop {
        println!("Please input your guess!");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Faid to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("fail to convert to num!");
                continue;
            },
        };

        println!("Your guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
