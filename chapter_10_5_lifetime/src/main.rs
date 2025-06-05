use std::fmt::Display;

struct ImportantExcept<'a> {
    part: &'a str,
}

impl <'a> ImportantExcept<'a> {
    fn level(&self) -> i32 { // impl生命周期中的省略
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str { // impl生命周期中的省略
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    {   //生命周期 r > x，编译器会检查
        // let r;               //---------------+--`a
        // {                          //               /
        //     let x = 5;        //---+--- `b     /
        //     // r = &x;             //   /           /
        // }                          //---+           /
        // println!("r: {}", r);      //               /
    }                              //----------------+

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(&string1.as_str(), string2); // string1转为&str类型
    println!("The longest string is {}", result);

    // struct生命周期
    let novel = String::from("Call me Ishmael. Some yesrs age...");

    let first_sentence = novel.split('.') // first_sentence生命周期23-28行
        .next()
        .expect("Could not found a '.'");
    let i = ImportantExcept { // i的生命周期是26-28行，引用&first_sentence生命周期大于i，不会报错
        part: &first_sentence,
    };
}

// 'a定义了一个生命周期, 使用相同的生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { //生命周期标注保证x，y生命周期比较短的那个
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// // 泛型参数类型, Trait Bound, 生命周期
fn longest_with_an_annmouncement<'a, T> (x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}