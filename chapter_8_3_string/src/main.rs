fn main() {
    // 字符串字面值初始化String类型
    // 1. to_string() 方法
    let data = "initial contents";
    let s = data.to_string();
    let s1 = "initial contects".to_string();
    // 2. String::from() 函数
    let hello = String::from("hello");
    let hello = String::from("您好");
    let hello = String::from("안녕하세요");
    let hello = String::from("Annyong haseyo");
    let hello = String::from("こんにちは");
    let hello = String::from("hej,hallå");
    let hello = String::from("halló");
    let hello = String::from("‏هتاف للترحيب, ‏أهلا");
    let hello = String::from("привет");
    let hello = String::from("здраво");
    let hello = String::from("γεία σας, γεία σου");

    // 更新String push_str()
    let mut s = String::from("foo");
    let s1 = String::from("bar");
    s.push_str(&s1); // 传入字符串切片,不会获得所有权
    println!("{}", s);
    println!("{}", s1);  // s1还有所有权
    // 更新String push()
    let mut s2 = String::from("lo");
    s2.push('l'); // 单个字符的类型是char,必须用单引号
    println!("{}", s2);

    // 拼接字符串
    let s11 = String::from("Hello, ");
    let s22 = String::from("World!");
    let s33 = s11 + &s22; // &s22 的引用，没有取得所有权， 取得了s11的所有权， 把所有权返回给s33
    println!("{}", s33);
    // println!("{}", s11);  //s11的所有权已经被移动到s33，已经失效
    println!("{}", s22);  // s22的所有权被保留

    // + 拼接多个字符串
    let l1 = String::from("tic");
    let l2 = String::from("tac");
    let l3 = String::from("toe");
    let l4 = l1 + "-" + &l2 + "-" + &l3;
    println!("{}", l4);
    // format! 宏拼接
    let l1 = String::from("tic");
    let s = format!("{}-{}-{}", l1, l2, l3);
    println!("{}", s);

    // String用索引访问报错
    let s1 = String::from("hello");
    // let h = s1[0]; // 报错

    // String的len方法
    let len = String::from("здраво").len();
    // Unicode标量值
    println!("{}", len); // 12

    let hello = "здраво";
    // let answer = &hello[0]; // 这种语言一个字母占两个字节，无法访问
    // з: 208,151

    // 以字节看待字符串的方式
    let w = "こんにちは";
    for b in w.bytes() {
        println!("{}", b);
    }

    // 以字形簇表示字符串，标准库中不支持，需要的话到crates.io上去找三方库

    // 切割String
    let hello = "здраво";
    let s = &hello[0..4]; // &hello[0..3] 允许是panic，切割时需要沿着字符边界
    println!("{}", s);
}
