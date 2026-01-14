# Day 03 - 函数与控制流

## 1. 函数 (Functions)

函数在 Rust 代码中随处可见。我们已经见过语言中最重要的函数之一：`main` 函数，它是许多程序的入口点。

### 定义函数
使用 `fn` 关键字。Rust 代码中的函数和变量名使用 *snake_case* 规范风格。

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

### 参数 (Parameters)
```rust
fn print_value(x: i32) {
    println!("The value of x is: {}", x);
}
```

### 返回值
函数可以向调用它的代码返回值。我们不在返回值前命名它们，但要在箭头 (`->`) 后声明它们的类型。
Rust 中，**函数体中最后一个表达式的值**将作为返回值（注意没有分号）。

```rust
fn five() -> i32 {
    5 // 注意这里没有分号，这是一个表达式
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

## 2. 控制流 (Control Flow)

### if 表达式
```rust
let number = 3;

if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```
`if` 是一个表达式，我们可以用它来赋值：
```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

### 循环 (Loops)
Rust 有三种循环：`loop`, `while`, 和 `for`。

1.  **loop**: 无限循环，直到显式停止。
    ```rust
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break 可以返回值
        }
    };
    ```

2.  **while**: 条件循环。
    ```rust
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    ```

3.  **for**: 遍历集合（最常用）。
    ```rust
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    // Range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    ```
