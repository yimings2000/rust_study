fn main() {
    let mut s1 = String::from("Hello");  // s1可变
    let len = calculate_length(&mut s1); // 传入的是s1的引用，所有权没有发生移动，是借用
    let _s2 = &mut s1;
    // let s3 = &mut s1; // &mut s1 不能两次引用, 会发生数据竞争
    // println!("{}, {}", _s2, s3);
    {
        let _s3 = &mut s1; // 作用域不同,可以同时创建多个可变引用
    } // s3 离开作用域
    println!("the length of {} is {}", s1, len);

    let mut s4 = String::from("Hello");
    let r1 = &s4;
    let r2 = &s4;
    // let r3 = &mut s4;  // 不可以同时有可变引用与不可变引用，对同一个变量
    // println!("{} {} {}", r1, r2, r3);

    // 悬空引用
    // let r = dangle();
}

fn calculate_length(s: &mut String) -> usize { // 传入s1的可变引用
    s.push_str(", world"); // s必须为可变引用才能修改
    s.len()
} // s离开作用域,不会drop,因为s不具有string的所有权,借用

// fn dangle() -> &String {  
//     let s = String::from("hello");
//     &s
// } // s出作用域失效,但是返回s的引用会出现悬空,编译器会报错，需要生命周期