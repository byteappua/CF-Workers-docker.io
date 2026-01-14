#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message called: {:?}", self);
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

fn main() {
    // 1. 枚举与数据
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    // 2. Option 枚举
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("absent_number: {:?}", absent_number);

    // 3. match 表达式
    let c1 = Coin::Penny;
    let c2 = Coin::Quarter(UsState::Alaska);

    println!("Value of penny: {}", value_in_cents(c1));
    println!("Value of quarter: {}", value_in_cents(c2));

    // 4. 匹配 Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
