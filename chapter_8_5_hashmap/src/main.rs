use std::collections::HashMap;
fn main() {
    // 创建HashMap方法，使用HashMap::new();
    let mut scores = HashMap::new(); // 可以显示指定类型scores: HashMap<String, i32>
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 创建HashMap方法, 使用collect
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![10, 50];
    let scores: HashMap<_, _> = 
        teams.iter().zip(intial_scores.iter()).collect();

    // HashMap和所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // println!("{}, {}", field_name, field_value);  // 所有权转移，此处失效
    map.insert(&field_name, &field_value);
    println!("{}: {}", field_name, field_value);  // 引用,所有权没有移动，所以没有失效

    // 访问HashMap中的值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("{}", s),
        None => println!("team not exist"),
    }

    // 遍历HashMap
    for (k, v)  in scores {
        println!("{}: {}", k, v);
    }

    // 更新HashMap中的数据: 替换现有的V
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Blue"), 25);
    println!("{:#?}", scores2);

    // 更新HashMap中的数据: 只在K不对应任何值的情况下, 才插入V
    let e = scores2.entry(String::from("Yellow"));
    println!("{:?}", e);  // 不存在V
    e.or_insert(50); // 不存在才会插入
    scores2.entry(String::from("Blue")).or_insert(50); // "Blue"存在,所以不会插入50
    println!("{:#?}", scores2);

    // 更新HashMap中的数据: 合并现有的V和新的V
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {  // 对text分割，按照空格分隔成单词
        let count = map.entry(word).or_insert(0); // 如果word不存在就插入，
        *count += 1; // 解引用，count + 1
    }
    println!("{:#?}", map);
}
