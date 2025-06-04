use std::fmt::{Display, Debug};
pub trait Summary {
    // 定义不实现, 下面必须要实现才能使用
    // fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;
    // 默认实现, 下面可以不实现直接使用
    fn summarize(&self) -> String {
        format!("(Read more from {} ...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.locaction)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String { // 不使用默认实现，相当于重写实现
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// impl trait语法
pub fn notify1(item1: impl Summary, item2: impl Summary) { // 参数的类型，实现了Summary trait 的类型
    println!("Breaking news! {}", item1.summarize());
}

// trait bound语法
pub fn notify2<T: Summary>(item1: T, item2: T) {
    println!("Breaking news! {}", item1.summarize());
}

// 使用 + 指定多个trait bound实现
// impl trait语法
pub fn notify3(item1: impl Summary + Display) { // 参数的类型，实现了Summary trait 的类型
    println!("Breaking news! {}", item1.summarize());
}

// trait bound语法
pub fn notify4<T: Summary + Display>(item1: T, item2: T) {
    println!("Breaking news! {}", item1.summarize());
}

// where 子句用法
pub fn notify5<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String { // 函数签名太长，不直观
    format!("Breaking news! {}", a.summarize())
}

pub fn notify6<T, U>(a: T, b: U) -> String
where 
    T: Summary + Display,
    U: Clone + Debug
{
    format!("Breaking news! {}", a.summarize())
}

// 返回类型实现Summary Trait
pub fn notify7(s: &str) -> impl Summary {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey
            team in the MHL."
        ),
        author: String::from("Iceburgh"),
        location: String::from("Pittsburgh, PA, USA"),
    }
}

struct Pair<T> {
    x: T,
    y: T,
}
// 所有类型都有new方法
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x: x, y: y}
    }
}

// 有条件的为实现了特定Trait的类型来实现方法
// 只有类型实现了Display + PartialOrd的Trait才可以拥有cmp_display()方法
impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}