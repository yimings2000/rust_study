use core::num;
use std::result;

fn largest(list: &[i32]) -> i32 { // list &[i32] 集合
    let mut largest = list[0];
    for &item in list {      // &item = i32, 可以之后与largest比较大小
        if item > largest {  // if *item > largest
            largest = item;  // largest = *item;
        }
    }
    largest
}

// 泛型函数改写
// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {  // : std::cmp::PartialOrd 实现这个trait才能比较
//     let mut largest = list[0];
//     for &item in list {     // &item = i32, 可以之后与largest比较大小
//         if item > largest { // if *item > largest
//             largest = item; // largest = *item;
//         }
//     }
//     largest
// }

// 结构体泛型
struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

// 枚举泛型
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 方法定义中的泛型
struct Point3<T> {
    x: T,
    y: T,
}

impl<T> Point3<T> { // impl<T> Point3<T>为所有类型的泛型T，实现的方法
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point3<i32> { // impl Point3<i32>只为i32类型实现的泛型方法
    fn x1(&self) -> &i32 {
        &self.x
    }
}

// struct里泛型类型参数可以和方法的泛型类型参数不同

struct Point4<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point4<T, U> {
    fn mixup<V, W>(self, other: Point4<V, W>) -> Point4<T, W> {
        Point4 { x: self.x, y: other.y }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    // 泛型
    // let number_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // 结构体泛型
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};

    // struct里泛型类型参数可以和方法的泛型类型参数不同
    let p1 = Point4 {x: 5, y: 4};
    let p2 = Point4 {x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}