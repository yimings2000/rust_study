use core::error;
use std::f32::consts::E;
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    // panic!("crash and burn");  // thread 'main' panicked at src\main.rs:4:5

    // let v = vec![1, 2, 3];
    // v[99];

    // Result match
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("error opening file {:?}", error)
    //     }
    // };

    // ErrorKing 使用多个match
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("error creating file {:?}", e),
    //         }
    //         oe => panic!("error opening the file: {:?}", oe),
    //     },
    // };

    // 使用闭包简化程序
    let f = File::open("hello.txt");
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error);
            })
        } else {
            panic!("Error opening file {:?}", error);
        }
    });

}
