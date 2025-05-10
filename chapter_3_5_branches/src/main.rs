fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    let condition = true;

    let num1 = if condition {5} else {6};

    println!("the value of num1 is {}", num1);

    // loop

    let mut conter = 0;
    let result = loop {
        conter += 1;
        if conter == 10 {
            break conter *2;
        }
    };
    println!("the result is : {}", result);

    // while

    let mut num2 = 3;
    while num2 != 0 {
        println!("{}!", num2);

        num2 = num2 - 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    // index < 5 容易越界, index < 6 编译成功，运行出错
    while index < 5 {
        println!("while the value is: {}", a[index]);
        index = index + 1;
    }

    // for 
    for element in a.iter() {
        println!("for the value is : {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
