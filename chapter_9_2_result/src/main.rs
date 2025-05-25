use std::f32::consts::E;
use std::fs::File;
use std::io;
use std::io::Read;
fn main() {
    let result = read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    // Result match 处理
    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // };
    
    // ? 运算符
    let mut f = File::open("hello.txt")?; // File::open的返回类型是Result，后面加个?相当于下面注释的代码
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    Ok(s)
}

// 链式调用
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}