fn main() {
    // 1. 函数调用
    print_sum(5, 6);

    let x = plus_one(5);
    println!("5 + 1 = {}", x);

    // 2. if 表达式
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4, 3");
    }

    // if let 赋值
    let condition = true;
    let val = if condition { 5 } else { 6 };
    println!("The value is: {}", val);

    // 3. 循环
    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result from loop is: {}", result);

    // while
    let mut n = 3;
    while n != 0 {
        println!("while: {}", n);
        n -= 1;
    }

    // for
    let a = [10, 20, 30, 40, 50];
    print!("for array: ");
    for element in a {
        print!("{} ", element);
    }
    println!();

    print!("for range: ");
    for number in 1..4 {
        print!("{} ", number);
    }
    println!();
}

fn print_sum(x: i32, y: i32) {
    println!("sum of {} + {} = {}", x, y, x + y);
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 表达式，返回值
}
