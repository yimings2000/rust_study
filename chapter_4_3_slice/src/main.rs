fn main() {
    let mut s1 = String::from("Hello world");
    let word_index1 = first_world_1(&s1);

    s1.clear(); // 如果把s clear,world_index还为5，与字符串没有关联，设计不好
    println!("{}", word_index1);

    let mut s2 = String::from("Hello world");
    let word_index2 = first_world_2(&s2);

    // s2.clear(); // 可变引用，前面first_world_2(&s2)已经被借用为不可变引用，会报错
    println!("{}", word_index2);

    // 字符串字面值的类型是&str
    let s3 = "hello world";

    let my_string = String::from("Hello world");
    let word_index3 = first_world_3(&my_string[..]);

    let my_string_literal = "hello world";
    let word_index3 = first_world_3(my_string_literal);
}

// 传入String, 返回len
fn first_world_1(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// 传入String 返回字符串切片&str
fn first_world_2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// 传入&str, 返回&str
fn first_world_3(s: &str) -> &str {
    // &str接受参数的好处是:既可以接收String类型也可以接收&str类型
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        println!("{}", item);
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}