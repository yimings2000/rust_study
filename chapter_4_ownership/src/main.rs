fn main() {
    let mut s1 = String::from("hello");
    //ex. 1
    // let s2 = s1;
    // println!("s1 = {}", s1); //错误:value borrowed here after move

    //ex. 2
    // let s2 = s1.clone();
    // println!("{}, {}", s1, s2);

    let len = calculate_lens(&mut s1);
    println!("{}, {}", s1, len);

    fn calculate_lens(s: &mut String) -> usize {
        s.push_str(", world!");
        s.len()
    }
}
