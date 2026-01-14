# Day 08 - 枚举 (Enums) 与 模式匹配 (Match)

枚举 (Enumerations) 允许你通过列举可能的**成员 (variants)** 来定义一个类型。

## 1. 定义枚举
```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

### 枚举成员的数据
Rust 的枚举非常强大，成员可以包含不同类型和数量的数据。
```rust
enum Message {
    Quit,                       // 没有关联数据
    Move { x: i32, y: i32 },    // 包含一个匿名结构体
    Write(String),              // 包含单独一个 String
    ChangeColor(i32, i32, i32), // 包含三个 i32
}
```

## 2. Option 枚举
Rust 没有 Null，而是使用 `Option<T>` 枚举来表示一个值可能存在，也可能不存在。
它定义在标准库中：
```rust
enum Option<T> {
    None,
    Some(T),
}
```

```rust
let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;
```

## 3. match 控制流运算符
`match` 允许我们将一个值与一系列的模式进行比较，并根据匹配的模式执行代码。它必须是**穷尽的 (exhaustive)**，即必须覆盖所有可能的情况。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### 绑定值的模式
`match` 分支可以绑定匹配到的模式的部分值。
```rust
enum Coin {
    Quarter(UsState), // 假设 UsState 是一个枚举
    // ...
}

match coin {
    Coin::Quarter(state) => {
        println!("State quarter from {:?}!", state);
        25
    }
    // ...
}
```

### 匹配 Option<T>
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

### 通配符 (_)
`_` 可以匹配所有未被显式列出的值。
```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}
```
